use super::*;
use crate::components::{rng_provider::*, *};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum HelmetAffixKind {
    MoreLife,
    AddArmour,
}

const HELMET_MORE_ARMOUR_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const HELMET_MORE_LIFE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

#[derive(Deref, DerefMut)]
struct HelmetAffixProvider(RngKindProvider<HelmetAffixKind>);

impl HelmetAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(
            HelmetAffixKind::AddArmour,
            HELMET_MORE_ARMOUR_RANGES.weight(ilevel),
        );
        provider.add(
            HelmetAffixKind::MoreLife,
            HELMET_MORE_LIFE_RANGES.weight(ilevel),
        );
        HelmetAffixProvider(provider)
    }
}

#[derive(Component)]
#[require(
    Name(|| Name::new("Helmet")),
    Equipment(|| Equipment::Helmet),
    ItemLevel
)]
pub struct Helmet;

impl EquipmentUI for Helmet {
    fn title() -> String {
        "Helmet".into()
    }

    fn tile_index(rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 182,
            ItemRarity::Magic => 184,
            ItemRarity::Rare => 185,
        }
    }
}

impl Helmet {
    pub fn spawn(commands: &mut Commands, ilevel: u16, rng: &mut ThreadRng) -> ItemEntityInfo {
        let mut provider = HelmetAffixProvider::new(ilevel);
        let mut helmet = AffixesInserter::spawn(commands, Helmet, ilevel, rng);
        for _ in 0..helmet.n_affix() {
            match provider.gen(rng) {
                Some(HelmetAffixKind::AddArmour) => {
                    helmet.insert::<Armour, u16>(HELMET_MORE_ARMOUR_RANGES.generate(ilevel, rng));
                }
                Some(HelmetAffixKind::MoreLife) => {
                    helmet.insert::<MoreLife, u16>(HELMET_MORE_LIFE_RANGES.generate(ilevel, rng));
                }
                None => {}
            }
        }
        helmet.equipment_entity()
    }
}
