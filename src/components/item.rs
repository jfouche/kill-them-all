use super::{rng_provider::RngKindProvider, EquipmentProvider};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

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
        match rng.gen_range(0..100) {
            0..40 => EquipmentProvider::new(self.0).spawn(commands, rng),
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

/// Utility to manage adding affix according to ilevel.
pub trait AffixConfigGenerator {
    fn max_affix_index(&self, ilevel: u16) -> usize;
    fn weight(&self, ilevel: u16) -> usize;
    fn generate(&self, ilevel: u16, rng: &mut ThreadRng) -> u16;
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

    fn generate(&self, ilevel: u16, rng: &mut ThreadRng) -> u16 {
        let max_idx = self.max_affix_index(ilevel);
        self[0..=max_idx]
            .choose(rng)
            .map(|(_, (min, max), _)| rng.gen_range(*min..=*max))
            .expect("Item affix levels must not be empty")
    }
}
