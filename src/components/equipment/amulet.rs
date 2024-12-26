use crate::components::{rng_provider::RngKindProvider, *};
use bevy::prelude::*;
use equipment::{AffixesInserter, EquipmentUI};
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum AmuletAffixKind {
    AddLife,
    AddArmour,
    PierceChance,
}

#[derive(Deref, DerefMut)]
struct AmuletAffixProvider(RngKindProvider<AmuletAffixKind>);

impl AmuletAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(AmuletAffixKind::AddArmour, 20);
        provider.add(AmuletAffixKind::AddLife, 20);
        provider.add(AmuletAffixKind::PierceChance, 10);
        AmuletAffixProvider(provider)
    }
}

#[derive(Component)]
#[require(
    Name(|| Name::new("Amulet"))
)]
pub struct Amulet;

impl EquipmentUI for Amulet {
    fn title() -> String {
        "Amulet".into()
    }

    fn tile_index(rarity: EquipmentRarity) -> usize {
        match rarity {
            EquipmentRarity::Normal => 213,
            EquipmentRarity::Magic => 215,
            EquipmentRarity::Rare => 216,
        }
    }
}

impl Amulet {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntityInfo {
        let mut provider = AmuletAffixProvider::new();
        let mut amulet = AffixesInserter::spawn(commands, Amulet, rng);
        for _ in 0..amulet.n_affix() {
            match provider.gen(rng) {
                Some(AmuletAffixKind::AddArmour) => {
                    amulet.insert::<Armour, u16>(rng.gen_range(1..=3));
                }
                Some(AmuletAffixKind::AddLife) => {
                    amulet.insert::<MoreLife, u16>(rng.gen_range(5..=10));
                }
                Some(AmuletAffixKind::PierceChance) => {
                    amulet.insert::<PierceChance, u16>(rng.gen_range(5..=10));
                }
                None => {}
            }
        }
        amulet.equipment_entity()
    }
}
