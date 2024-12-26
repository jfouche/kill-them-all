use super::*;
use crate::components::{rng_provider::*, *};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum BootsAffixKind {
    AddLife,
    AddArmour,
    IncreaseMovementSpeed,
}

#[derive(Deref, DerefMut)]
struct BootsAffixProvider(RngKindProvider<BootsAffixKind>);

impl BootsAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(BootsAffixKind::AddArmour, 20);
        provider.add(BootsAffixKind::AddLife, 20);
        provider.add(BootsAffixKind::IncreaseMovementSpeed, 20);
        BootsAffixProvider(provider)
    }
}

#[derive(Component)]
#[require(
    Name(|| Name::new("Boots"))
)]
pub struct Boots;

impl EquipmentUI for Boots {
    fn title() -> String {
        "Boots".into()
    }

    fn tile_index(rarity: EquipmentRarity) -> usize {
        match rarity {
            EquipmentRarity::Normal => 63,
            EquipmentRarity::Magic => 65,
            EquipmentRarity::Rare => 66,
        }
    }
}

impl Boots {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntityInfo {
        let mut provider = BootsAffixProvider::new();
        let mut boots = AffixesInserter::spawn(commands, Boots, rng);
        for _ in 0..boots.n_affix() {
            match provider.gen(rng) {
                Some(BootsAffixKind::AddArmour) => {
                    boots.insert::<Armour, u16>(rng.gen_range(1..=3));
                }
                Some(BootsAffixKind::AddLife) => {
                    boots.insert::<MoreLife, u16>(rng.gen_range(5..=10));
                }
                Some(BootsAffixKind::IncreaseMovementSpeed) => {
                    boots.insert::<IncreaseMovementSpeed, u16>(rng.gen_range(5..=30));
                }
                None => {}
            }
        }
        boots.equipment_entity()
    }
}
