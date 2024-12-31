use super::*;
use crate::components::{rng_provider::*, *};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum HelmetAffixKind {
    MoreLife,
    AddArmour,
}

#[derive(Deref, DerefMut)]
struct HelmetAffixProvider(RngKindProvider<HelmetAffixKind>);

impl HelmetAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(HelmetAffixKind::AddArmour, 20);
        provider.add(HelmetAffixKind::MoreLife, 20);
        HelmetAffixProvider(provider)
    }
}

#[derive(Component)]
#[require(
    Name(|| Name::new("Helmet")),
    Equipment(|| Equipment::Helmet)
)]
pub struct Helmet;

impl EquipmentUI for Helmet {
    fn title() -> String {
        "Helmet".into()
    }

    fn tile_index(rarity: EquipmentRarity) -> usize {
        match rarity {
            EquipmentRarity::Normal => 182,
            EquipmentRarity::Magic => 184,
            EquipmentRarity::Rare => 185,
        }
    }
}

impl Helmet {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntityInfo {
        let mut provider = HelmetAffixProvider::new();
        let mut helmet = AffixesInserter::spawn(commands, Helmet, rng);
        for _ in 0..helmet.n_affix() {
            match provider.gen(rng) {
                Some(HelmetAffixKind::AddArmour) => {
                    helmet.insert::<Armour, u16>(rng.gen_range(1..=3));
                }
                Some(HelmetAffixKind::MoreLife) => {
                    helmet.insert::<MoreLife, u16>(rng.gen_range(5..=10));
                }
                None => {}
            }
        }
        helmet.equipment_entity()
    }
}
