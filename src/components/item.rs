use super::{equipment::EquipmentProvider, orb::OrbProvider, rng_provider::RngKindProvider};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

pub const ITEM_SIZE: UVec2 = UVec2::new(48, 48);

#[derive(Resource)]
pub struct ItemAssets {
    texture: Handle<Image>,
    atlas_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for ItemAssets {
    fn from_world(world: &mut World) -> Self {
        ItemAssets {
            texture: world.load_asset(
                "items/Kyrise's 16x16 RPG Icon Pack - V1.3/spritesheet/spritesheet_48x48.png",
            ),
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
pub struct Item;

/// Item dropped by a monster.
///
/// It reference the [Item] entity
#[derive(Component, Copy, Clone, Deref, Reflect)]
#[require(
    Name(|| Name::new("DroppedItem")),
    Sprite,
)]
pub struct DroppedItem(pub Entity);

#[derive(Component, Default, Deref, Reflect)]
#[require(Item)]
pub struct ItemLevel(pub u16);

/// Provide a random [Item], based on the level provided
pub struct ItemProvider(pub u16);

impl ItemProvider {
    pub fn spawn(&self, commands: &mut Commands, rng: &mut ThreadRng) -> Option<ItemEntityInfo> {
        match rng.random_range(0..100) {
            0..40 => EquipmentProvider::new(self.0).spawn(commands, rng),
            40..80 => OrbProvider::new().spawn(commands, rng),
            _ => None,
        }
    }
}

pub struct ItemEntityInfo {
    pub entity: Entity,
    pub info: ItemInfo,
}

/// Util component to store all equipments informations, e.g. image and affixes

#[derive(Component, Default, Clone, Reflect)]
pub struct ItemInfo {
    pub tile_index: usize,
    pub text: String,
}

impl From<&ItemInfo> for Text {
    fn from(value: &ItemInfo) -> Self {
        Text(value.text.clone())
    }
}

/// Equipment Rarity
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ItemRarity {
    Normal,
    Magic,
    Rare,
}

impl ItemRarity {
    pub fn n_affix(&self) -> u16 {
        match self {
            ItemRarity::Normal => 1,
            ItemRarity::Magic => 2,
            ItemRarity::Rare => 3,
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct ItemRarityProvider(RngKindProvider<ItemRarity>);

impl ItemRarityProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(ItemRarity::Normal, 10);
        provider.add(ItemRarity::Magic, 8);
        provider.add(ItemRarity::Rare, 5);
        ItemRarityProvider(provider)
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
