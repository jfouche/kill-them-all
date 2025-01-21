use super::{AffixesInserter, Equipment, EquipmentUI};
use crate::components::{
    affix::{Armour, MoreLife, PierceChance},
    item::{AffixConfigGenerator, ItemEntityInfo, ItemLevel, ItemRarity},
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum AmuletAffixKind {
    MoreLife,
    AddArmour,
    PierceChance,
}

const AMULET_MORE_ARMOUR_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const AMULET_MORE_LIFE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const AMULET_PIERCE_CHANCE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 10), (10, (10, 24), 10), (17, (25, 29), 10)];

#[derive(Deref, DerefMut)]
struct AmuletAffixProvider(RngKindProvider<AmuletAffixKind>);

impl AmuletAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();

        provider.add(
            AmuletAffixKind::AddArmour,
            AMULET_MORE_ARMOUR_RANGES.weight(ilevel),
        );
        provider.add(
            AmuletAffixKind::MoreLife,
            AMULET_MORE_LIFE_RANGES.weight(ilevel),
        );
        provider.add(
            AmuletAffixKind::PierceChance,
            AMULET_PIERCE_CHANCE_RANGES.weight(ilevel),
        );

        AmuletAffixProvider(provider)
    }
}

#[derive(Component)]
#[require(
    Name(|| Name::new("Amulet")),
    Equipment(|| Equipment::Amulet),
    ItemLevel
)]
pub struct Amulet;

impl EquipmentUI for Amulet {
    fn title() -> String {
        "Amulet".into()
    }

    fn tile_index(rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 213,
            ItemRarity::Magic => 215,
            ItemRarity::Rare => 216,
        }
    }
}

impl Amulet {
    pub fn spawn(commands: &mut Commands, ilevel: u16, rng: &mut ThreadRng) -> ItemEntityInfo {
        let mut provider = AmuletAffixProvider::new(ilevel);
        let mut amulet = AffixesInserter::spawn(commands, Amulet, ilevel, rng);
        for _ in 0..amulet.n_affix() {
            match provider.gen(rng) {
                Some(AmuletAffixKind::AddArmour) => {
                    amulet.insert::<Armour, _>(AMULET_MORE_ARMOUR_RANGES.generate(ilevel, rng));
                }
                Some(AmuletAffixKind::MoreLife) => {
                    amulet.insert::<MoreLife, _>(AMULET_MORE_LIFE_RANGES.generate(ilevel, rng));
                }
                Some(AmuletAffixKind::PierceChance) => {
                    amulet.insert::<PierceChance, _>(
                        AMULET_PIERCE_CHANCE_RANGES.generate(ilevel, rng),
                    );
                }
                None => {}
            }
        }
        amulet.equipment_entity()
    }
}
