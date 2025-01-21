use super::{AffixesInserter, Equipment, EquipmentUI};
use crate::components::{
    affix::{Armour, IncreaseMovementSpeed, MoreLife},
    item::{AffixConfigGenerator, ItemEntityInfo, ItemLevel, ItemRarity},
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum BootsAffixKind {
    AddLife,
    AddArmour,
    IncreaseMovementSpeed,
}

const BOOTS_MORE_ARMOUR_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const BOOTS_MORE_LIFE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const BOOTS_INCR_MOVEMENT_SPEED_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

#[derive(Deref, DerefMut)]
struct BootsAffixProvider(RngKindProvider<BootsAffixKind>);

impl BootsAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(
            BootsAffixKind::AddArmour,
            BOOTS_MORE_ARMOUR_RANGES.weight(ilevel),
        );
        provider.add(
            BootsAffixKind::AddLife,
            BOOTS_MORE_LIFE_RANGES.weight(ilevel),
        );
        provider.add(
            BootsAffixKind::IncreaseMovementSpeed,
            BOOTS_INCR_MOVEMENT_SPEED_RANGES.weight(ilevel),
        );
        BootsAffixProvider(provider)
    }
}

#[derive(Component)]
#[require(
    Name(|| Name::new("Boots")),
    Equipment(|| Equipment::Boots),
    ItemLevel
)]
pub struct Boots;

impl EquipmentUI for Boots {
    fn title() -> String {
        "Boots".into()
    }

    fn tile_index(rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 63,
            ItemRarity::Magic => 65,
            ItemRarity::Rare => 66,
        }
    }
}

impl Boots {
    pub fn spawn(commands: &mut Commands, ilevel: u16, rng: &mut ThreadRng) -> ItemEntityInfo {
        let mut provider = BootsAffixProvider::new(ilevel);
        let mut boots = AffixesInserter::spawn(commands, Boots, ilevel, rng);
        for _ in 0..boots.n_affix() {
            match provider.gen(rng) {
                Some(BootsAffixKind::AddArmour) => {
                    boots.insert::<Armour, _>(BOOTS_MORE_ARMOUR_RANGES.generate(ilevel, rng));
                }
                Some(BootsAffixKind::AddLife) => {
                    boots.insert::<MoreLife, _>(BOOTS_MORE_LIFE_RANGES.generate(ilevel, rng));
                }
                Some(BootsAffixKind::IncreaseMovementSpeed) => {
                    boots.insert::<IncreaseMovementSpeed, _>(
                        BOOTS_INCR_MOVEMENT_SPEED_RANGES.generate(ilevel, rng),
                    );
                }
                None => {}
            }
        }
        boots.equipment_entity()
    }
}
