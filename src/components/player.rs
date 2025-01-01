use super::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
#[require(
    Name(|| Name::new("Player")),
    Character,
    Target(|| Target::Monster),
    BaseLife(|| BaseLife(10.)),
    BaseMovementSpeed(|| BaseMovementSpeed(130.)),
    Experience,
    Sprite,
    Transform(|| Transform::from_xyz(0., 0., 10.)),
    AnimationTimer,
    Collider(|| Collider::cuboid(PLAYER_SIZE.x / 2., PLAYER_SIZE.y / 2.)),
    CollisionGroups(|| CollisionGroups::new(GROUP_PLAYER, GROUP_ALL)),
    ActiveEvents(|| ActiveEvents::COLLISION_EVENTS)
)]
pub struct Player;

impl Player {
    pub fn sprite(assets: &PlayerAssets) -> Sprite {
        Sprite {
            image: assets.texture.clone(),
            texture_atlas: Some(assets.atlas_layout.clone().into()),
            custom_size: Some(PLAYER_SIZE),
            ..Default::default()
        }
    }
}

pub const PLAYER_SIZE: Vec2 = Vec2::new(16.0, 16.0);

/// All [Player] assets
#[derive(Resource)]
pub struct PlayerAssets {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let texture = world.load_asset("characters/RedNinja/SpriteSheet.png");
        let atlas_layout = world.add_asset(TextureAtlasLayout::from_grid(
            UVec2::new(16, 16),
            4,
            7,
            None,
            None,
        ));

        PlayerAssets {
            texture,
            atlas_layout,
        }
    }
}

/// Event to notify the player died
#[derive(Event)]
pub struct PlayerDeathEvent;

/// Event to notify a player level up
#[derive(Event)]
pub struct LevelUpEvent;

// ==================================================================
// Experience

#[derive(Component, Default, Debug, Reflect)]
pub struct Experience(u32);

impl Experience {
    const LEVELS: [u32; 6] = [4, 10, 30, 80, 170, 300];

    pub fn add(&mut self, xp: u32) {
        self.0 += xp;
    }

    pub fn current(&self) -> u32 {
        self.0
    }

    /// Level starting at 0
    pub fn level(&self) -> u8 {
        let mut level = 0;
        for xp in Experience::LEVELS.iter() {
            if self.0 >= *xp {
                level += 1;
            } else {
                break;
            }
        }
        level
    }

    pub fn get_current_level_min_max_exp(&self) -> (u32, u32) {
        let level = self.level();
        let min = match level {
            0 => 0,
            _ => Self::LEVELS.get(level as usize - 1).cloned().unwrap_or(0),
        };
        let max = Self::LEVELS
            .get(level as usize)
            .cloned()
            .unwrap_or(*Self::LEVELS.last().unwrap());
        (min, max)
    }
}

impl std::fmt::Display for Experience {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{} (level {})",
            self.0,
            self.get_current_level_min_max_exp().1,
            self.level() + 1,
        )
    }
}

///
/// Resource to store the score
///
#[derive(Default, Resource)]
pub struct Score(pub u16);

///
/// The [Inventory] contains all items that carry the [Player] as children
///
#[derive(Component)]
#[require(Name(|| Name::new("Inventory")))]
pub struct Inventory;

/// Event to indicate The [Inventory] changed
#[derive(Event)]
pub struct InventoryChanged;

/// Event to indicate The [Player] equipments changed
#[derive(Event)]
pub struct PlayerEquipmentChanged;

/// Command to add an [Equipment] to the [Player].
///
/// If the item is in the [Inventory], it will remove it.
/// If the [Player] already have this kind of [Equipment], it will put the old
/// one to the [Inventory]
pub struct EquipItemCommand(pub Entity);

impl Command for EquipItemCommand {
    fn apply(self, world: &mut World) {
        let player = world.query_filtered::<Entity, With<Player>>().single(world);
        let inventory = world
            .query_filtered::<Entity, With<Inventory>>()
            .single(world);

        let mut inventory_changed = world
            .query::<&Parent>()
            .get(world, self.0)
            .map(|parent| **parent == inventory)
            .unwrap_or(false);

        // Check it the player already have an item of same type
        let mut equipments = world.query::<(Entity, &Equipment, &Parent)>();
        let current_equipment = equipments
            .get(world, self.0)
            .map(|(_, eqp, _)| *eqp)
            .expect("Equipment");

        let old_equipments = equipments
            .iter(world)
            // same parent, same type, but different entity
            .filter(|(entity, eqp, parent)| {
                warn!(" * filter({entity}, {:?}, {})", eqp, ***parent);
                player == ***parent && **eqp == current_equipment && *entity != self.0
            })
            .map(|(e, _eqp, _p)| e)
            .collect::<Vec<_>>();
        warn!("old_equipments = {old_equipments:?}");
        if !old_equipments.is_empty() {
            // Move old equipments (should be single) in inventory
            world.entity_mut(inventory).add_children(&old_equipments);
            inventory_changed = true;
        }

        // Add_child will remove the old parent before applying new parenting
        world.entity_mut(player).add_child(self.0);

        // Flush the world to ensure the items are correctly attached to
        // their parent before triggering events
        world.flush();

        world.trigger(PlayerEquipmentChanged);
        if inventory_changed {
            world.trigger(InventoryChanged);
        }
    }
}

/// Command to drop an item
pub struct DropItemCommand(pub Entity);

impl Command for DropItemCommand {
    fn apply(self, world: &mut World) {
        let player = world.query_filtered::<Entity, With<Player>>().single(world);
        let inventory = world
            .query_filtered::<Entity, With<Inventory>>()
            .single(world);

        enum Change {
            None,
            Player,
            Inventory,
            Other(Entity),
        }

        let change = world
            .query::<&Parent>()
            .get(world, self.0)
            .map(|p| {
                if **p == player {
                    Change::Player
                } else if **p == inventory {
                    Change::Inventory
                } else {
                    Change::Other(**p)
                }
            })
            .unwrap_or(Change::None);

        match change {
            Change::Player => {
                world.commands().entity(player).remove_children(&[self.0]);
            }
            Change::Inventory => {
                world
                    .commands()
                    .entity(inventory)
                    .remove_children(&[self.0]);
            }
            Change::Other(parent) => {
                world.commands().entity(parent).remove_children(&[self.0]);
            }
            Change::None => {}
        }
        world.despawn(self.0);
        world.flush();

        match change {
            Change::Player => {
                world.trigger(PlayerEquipmentChanged);
            }
            Change::Inventory => {
                world.trigger(InventoryChanged);
            }
            Change::Other(_) => {}
            Change::None => {}
        }
    }
}
