use super::{
    animation::AnimationTimer,
    character::{BaseLife, BaseMovementSpeed, Character, Target},
    inventory::{AddToInventoryCommand, PlayerEquipmentChanged, RemoveFromInventoryCommand},
    skills::SkillGem,
    GROUP_ALL, GROUP_PLAYER,
};
use crate::utils::despawn_after::DespawnAfter;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

#[derive(Component)]
#[require(
    Name(|| Name::new("Player")),
    Character,
    PlayerSkills,
    Target(|| Target::Monster),
    BaseLife(|| BaseLife(10.)),
    BaseMovementSpeed(|| BaseMovementSpeed(100.)),
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

#[derive(Component, Clone, Copy)]
pub enum PlayerAction {
    Skill1,
    Skill2,
    Skill3,
    Skill4,
}

impl PlayerAction {
    fn index(&self) -> usize {
        match self {
            PlayerAction::Skill1 => 0,
            PlayerAction::Skill2 => 1,
            PlayerAction::Skill3 => 2,
            PlayerAction::Skill4 => 3,
        }
    }
}

impl From<usize> for PlayerAction {
    fn from(value: usize) -> Self {
        match value {
            0 => PlayerAction::Skill1,
            1 => PlayerAction::Skill2,
            2 => PlayerAction::Skill3,
            3 => PlayerAction::Skill4,
            _ => unreachable!(),
        }
    }
}

#[derive(Component, Default, Reflect)]
pub struct PlayerSkills([Option<Entity>; 4]);

impl PlayerSkills {
    // fn contains(&self, skill: Entity) -> Option<PlayerAction> {
    //     self.0
    //         .iter()
    //         .position(|&o| o == Some(skill))
    //         .map(|i| i.into())
    // }

    pub fn get(&self, action: PlayerAction) -> Option<Entity> {
        *self.0.get(action.index())?
    }

    fn set(&mut self, action: PlayerAction, skill: Entity) -> bool {
        let index = action.index();
        match self.0[index] {
            Some(_) => {
                warn!("Can't set skills[{index}] as it's not empty");
                false
            }
            None => {
                self.0[index] = Some(skill);
                true
            }
        }
    }

    pub fn remove(&mut self, skill: Entity) -> bool {
        let Some(index) = self.0.iter().position(|&o| o == Some(skill)) else {
            return false;
        };
        self.0.get_mut(index).map(|o| *o = None).is_some()
    }
}

pub struct EquipSkillGemCommand(pub Entity, pub PlayerAction);

impl Command for EquipSkillGemCommand {
    fn apply(self, world: &mut World) {
        let gem_entity = self.0;
        let mut skill_gems = world.query::<&SkillGem>();
        let Ok(_) = skill_gems.get(world, gem_entity) else {
            warn!("Can't equip {gem_entity} as it's not an SkillGem");
            return;
        };

        let Ok((player_entity, mut skills)) = world
            .query_filtered::<(Entity, &mut PlayerSkills), With<Player>>()
            .get_single_mut(world)
        else {
            error!("Player doesn't have a PlayerSkills");
            return;
        };

        let action = self.1;
        let old_gem = match skills.get(action) {
            Some(gem) => (gem != gem_entity).then_some(gem),
            None => None,
        };
        skills.set(action, gem_entity);

        // Manage inventory
        RemoveFromInventoryCommand(gem_entity).apply(world);
        if let Some(old_gem) = old_gem {
            AddToInventoryCommand(old_gem).apply(world);
        }

        // Add_child will remove the old parent before applying new parenting
        world.entity_mut(player_entity).add_child(gem_entity);
        world.trigger(PlayerEquipmentChanged);
    }
}

pub struct RemoveSkillGemCommand(pub Entity);

impl Command for RemoveSkillGemCommand {
    fn apply(self, world: &mut World) {
        let gem_entity = self.0;
        let mut skill_gems = world.query::<&SkillGem>();
        let Ok(_) = skill_gems.get(world, gem_entity) else {
            warn!("Can't remove {gem_entity} as it's not an SkillGem");
            return;
        };

        let Ok(mut skills) = world
            .query_filtered::<&mut PlayerSkills, With<Player>>()
            .get_single_mut(world)
        else {
            error!("Player doesn't have a PlayerSkills");
            return;
        };

        if skills.remove(gem_entity) {
            world.trigger(PlayerEquipmentChanged);
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

    /// Add xp
    pub fn add(&mut self, xp: u32) {
        self.0 += xp;
    }

    pub fn current(&self) -> u32 {
        self.0
    }

    /// Level starting at 0
    pub fn level(&self) -> u16 {
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
/// An indicator to show the next position of the [Player] on the map
///
#[derive(Component)]
#[require(
    Name(|| Name::new("NextPositionIndicator")),
    Mesh2d,
    MeshMaterial2d<ColorMaterial>,
    Transform,
    DespawnAfter(|| DespawnAfter::new(Duration::from_millis(300)))
)]
pub struct NextPositionIndicator;

///
/// Assets for [NextPositionIndicator]
///
#[derive(Resource)]
pub struct NextPositionIndicatorAssets {
    pub mesh: Handle<Mesh>,
    pub color: Handle<ColorMaterial>,
}

impl FromWorld for NextPositionIndicatorAssets {
    fn from_world(world: &mut World) -> Self {
        let mesh = world.add_asset(Circle::new(5.0));
        let color = world.add_asset(Color::srgba(1., 0., 0., 0.8));
        NextPositionIndicatorAssets { mesh, color }
    }
}
