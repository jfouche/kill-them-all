use super::{
    equipment::EquipmentProvider,
    inventory::{Inventory, InventoryChanged, PlayerEquipmentChanged},
    orb::{OrbAction, OrbProvider},
    player::Player,
    rng_provider::RngKindProvider,
    skills::SkillProvider,
};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};
use std::marker::PhantomData;

pub const ITEM_SIZE: UVec2 = UVec2::new(48, 48);

#[derive(Resource)]
pub struct ItemAssets {
    texture: Handle<Image>,
    atlas_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for ItemAssets {
    fn from_world(world: &mut World) -> Self {
        ItemAssets {
            texture: world.load_asset("kte-items.png"),
            atlas_layout: world
                .add_asset(TextureAtlasLayout::from_grid(ITEM_SIZE, 16, 22, None, None)),
        }
    }
}

impl ItemAssets {
    pub fn image(&self) -> Handle<Image> {
        self.texture.clone()
    }

    pub fn texture_atlas(&self, index: usize) -> TextureAtlas {
        TextureAtlas {
            layout: self.atlas_layout.clone(),
            index,
        }
    }

    pub fn image_node(&self, index: usize) -> ImageNode {
        ImageNode::from_atlas_image(self.image(), self.texture_atlas(index))
    }

    pub fn empty_image_node(&self) -> ImageNode {
        self.image_node(351)
    }

    pub fn sprite(&self, index: usize) -> Sprite {
        Sprite {
            image: self.image(),
            texture_atlas: Some(self.texture_atlas(index)),
            ..Default::default()
        }
    }
}

#[derive(Component, Default)]
#[require(ItemTitle, ItemDescription, ItemTileIndex)]
pub struct Item;

#[derive(Component, Default, Reflect)]
pub struct ItemTitle(pub String);

#[derive(Component, Default, Reflect)]
pub struct ItemDescription(pub String);

#[derive(Component, Default, Reflect)]
pub struct ItemTileIndex(pub usize);

/// Item dropped by a monster.
///
/// It reference the [Item] entity
#[derive(Component, Copy, Clone, Deref, Reflect)]
#[require(Name::new("DroppedItem"), Sprite)]
pub struct DroppedItem(pub Entity);

#[derive(Component, Clone, Copy, Default, Deref, Reflect)]
#[require(Item)]
pub struct ItemLevel(pub u16);

/// Component to add to UI to indicate which entity (if any) correspond to the node
#[derive(Component, Default, Reflect)]
pub struct ItemEntity(pub Option<Entity>);

/// Location of an [Item] in an [Item] container, like the player equipments or the
/// [crate::components::inventory::Inventory] for examples
#[derive(Component, Default)]
#[require(
    Node = ItemLocation::default_node(),
    BackgroundColor,
    ItemEntity
)]
pub struct ItemLocation;

impl ItemLocation {
    pub fn default_node() -> Node {
        Node {
            padding: UiRect::all(Val::Px(3.0)),
            ..Default::default()
        }
    }
}

#[derive(Component)]
pub struct ItemLocationAccept<T>(PhantomData<T>);

impl<T> ItemLocationAccept<T> {
    pub fn new() -> Self {
        ItemLocationAccept(PhantomData)
    }
}

#[derive(Component)]
pub struct ItemLocationAcceptAll;

#[derive(Component)]
#[require(
    ImageNode,
    BackgroundColor(Srgba::rgb(0.25, 0.25, 0.25).into()),
)]
pub struct ItemImage;

/// Provide a random [Item], based on the level provided
pub struct ItemProvider(pub u16);

impl ItemProvider {
    pub fn spawn(&self, commands: &mut Commands, rng: &mut ThreadRng) -> Option<Entity> {
        let entity = match rng.random_range(0..100) {
            0..30 => EquipmentProvider::new(self.0).spawn(commands, rng)?,
            30..60 => OrbProvider::spawn(commands, rng),
            60..90 => SkillProvider::new(self.0).spawn(commands, rng)?,
            _ => return None,
        };
        Some(entity)
    }
}

pub trait ItemSpawnBundle {
    type Implicit: Component + std::fmt::Display;
    fn new(ilevel: u16, rng: &mut ThreadRng) -> (Self, Self::Implicit)
    where
        Self: Sized;
}

/// Util to spawn a random [Item] of a given type.
///
/// The item can be [ItemRarity::Normal] or [ItemRarity::Rare]
pub struct ItemSpawner {
    pub ilevel: u16,
    pub rarity: ItemRarity,
}

impl ItemSpawner {
    pub fn new(ilevel: u16, rng: &mut ThreadRng) -> Self {
        Self {
            ilevel,
            rarity: ItemRarityProvider::gen(rng),
        }
    }

    /// Spawn a random item of type `T`.
    pub fn spawn<T>(&self, commands: &mut Commands, rng: &mut ThreadRng) -> Entity
    where
        T: Component + ItemSpawnBundle + ItemDescriptor + OrbAction,
    {
        let (mut item, implicit) = T::new(self.ilevel, rng);
        let mut item_cmds = commands.spawn_empty();
        let item_entity = item_cmds.id();
        item.add_affixes(&mut item_cmds, self.rarity.n_affix(), rng);
        item_cmds.insert((item, implicit, self.rarity));
        commands.queue(UpdateItemInfo::<T>::new(item_entity));
        item_entity
    }
}

pub trait ItemDescriptor {
    fn title(&self) -> String;
    fn description(&self) -> String;
    fn tile_index(&self, rarity: ItemRarity) -> usize;
}

pub struct UpdateItemInfo<T> {
    item_entity: Entity,
    _marker: PhantomData<T>,
}

impl<T> UpdateItemInfo<T> {
    pub fn new(item_entity: Entity) -> Self {
        UpdateItemInfo {
            item_entity,
            _marker: PhantomData,
        }
    }
}

impl<T> Command<Result> for UpdateItemInfo<T>
where
    T: Component + ItemDescriptor,
{
    fn apply(self, world: &mut World) -> Result {
        let (item, rarity, mut title, mut description, mut tile_index, child_of) = world
            .query::<(
                &T,
                Option<&ItemRarity>,
                &mut ItemTitle,
                &mut ItemDescription,
                &mut ItemTileIndex,
                Option<&ChildOf>,
            )>()
            .get_mut(world, self.item_entity)?;
        let rarity = rarity.copied().unwrap_or(ItemRarity::Normal);
        title.0 = item.title();
        description.0 = item.description();
        tile_index.0 = item.tile_index(rarity);

        if let Some(&ChildOf(parent)) = child_of {
            let mut query = world.query_filtered::<&Inventory, With<Player>>();
            if query.get(world, parent).is_ok() {
                world.trigger(PlayerEquipmentChanged);
            } else if let Ok(inventory) = query.single(world) {
                if inventory.contains(self.item_entity) {
                    world.trigger(InventoryChanged);
                }
            }
        }

        world.trigger(ItemChanged(self.item_entity));
        Ok(())
    }
}

pub fn update_item_info<T>() -> impl Fn(Trigger<OnAdd, T>, Commands)
where
    T: Component + ItemDescriptor,
{
    |trigger: Trigger<OnAdd, T>, mut commands: Commands| {
        commands.queue(UpdateItemInfo::<T>::new(trigger.target()));
    }
}

/// Event triggered when an [Item] has been modified (by an [Orb] for example)
#[derive(Event)]
pub struct ItemChanged(pub Entity);

/// Equipment Rarity
#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ItemRarity {
    #[default]
    Normal,
    Magic,
    Rare,
}

impl ItemRarity {
    pub fn n_affix(&self) -> u16 {
        match self {
            ItemRarity::Normal => 0,
            ItemRarity::Magic => 1,
            ItemRarity::Rare => 2,
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct ItemRarityProvider(RngKindProvider<ItemRarity>);

impl ItemRarityProvider {
    fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(ItemRarity::Normal, 10);
        provider.add(ItemRarity::Magic, 8);
        provider.add(ItemRarity::Rare, 5);
        ItemRarityProvider(provider)
    }

    pub fn gen(rng: &mut ThreadRng) -> ItemRarity {
        Self::new().0.gen(rng).expect("At least one rarity")
    }
}

pub struct ValueAndTier(pub u16, pub u8);

/// Utility to manage adding affix according to ilevel.
pub trait AffixConfigGenerator {
    fn max_affix_index(&self, ilevel: u16) -> usize;
    fn weight(&self, ilevel: u16) -> usize;
    /// Generate a (value, tier) from available affixes for the given `ilevel`
    fn generate(&self, ilevel: u16, rng: &mut ThreadRng) -> ValueAndTier;
}

/// impl for [(max_ilevel, (min_range, max_range), weight)] slice
impl<const N: usize> AffixConfigGenerator for [(u16, (u16, u16), usize); N] {
    fn max_affix_index(&self, ilevel: u16) -> usize {
        self.iter()
            .position(|(l, _, _)| ilevel <= *l)
            .unwrap_or(self.len().saturating_sub(1))
    }

    fn weight(&self, ilevel: u16) -> usize {
        self[0..=self.max_affix_index(ilevel)]
            .iter()
            .map(|(_, _, w)| w)
            .sum()
    }

    fn generate(&self, ilevel: u16, rng: &mut ThreadRng) -> ValueAndTier {
        let max_idx = self.max_affix_index(ilevel);
        let idx = rng.random_range(0..=max_idx);
        let tier = u8::try_from(self.len() - idx).expect("tier should be compatible with u8");
        let value = self
            .get(idx)
            .map(|(_, (min, max), _)| rng.random_range(*min..=*max))
            .expect("Item affix levels must not be empty");
        ValueAndTier(value, tier)
    }
}

/// Event to add an [crate::components::equipment::Equipment] to the [crate::components::player::Player].
#[derive(Event)]
pub struct EquipEquipmentEvent(pub Entity);

/// Event to drop an item
#[derive(Event)]
pub struct DropItemEvent(pub Entity);
