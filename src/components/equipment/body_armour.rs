use super::*;
use crate::components::{rng_provider::*, *};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum BodyArmourAffixKind {
    AddLife,
    AddArmour,
}

const BODYARMOUR_MORE_ARMOUR_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const BODYARMOUR_MORE_LIFE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

#[derive(Deref, DerefMut)]
struct BodyArmourAffixProvider(RngKindProvider<BodyArmourAffixKind>);

impl BodyArmourAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(
            BodyArmourAffixKind::AddArmour,
            BODYARMOUR_MORE_ARMOUR_RANGES.weight(ilevel),
        );
        provider.add(
            BodyArmourAffixKind::AddLife,
            BODYARMOUR_MORE_LIFE_RANGES.weight(ilevel),
        );
        BodyArmourAffixProvider(provider)
    }
}

#[derive(Component)]
#[require(
    Name(|| Name::new("BodyArmour")),
    Equipment(|| Equipment::BodyArmour),
    ItemLevel
)]
pub struct BodyArmour;

impl EquipmentUI for BodyArmour {
    fn title() -> String {
        "Body armour".into()
    }
    fn tile_index(rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 0,
            ItemRarity::Magic => 2,
            ItemRarity::Rare => 3,
        }
    }
}

impl BodyArmour {
    pub fn spawn(commands: &mut Commands, ilevel: u16, rng: &mut ThreadRng) -> ItemEntityInfo {
        let mut provider = BodyArmourAffixProvider::new(ilevel);
        let mut body_armour = AffixesInserter::spawn(commands, BodyArmour, ilevel, rng);
        for _ in 0..body_armour.n_affix() {
            match provider.gen(rng) {
                Some(BodyArmourAffixKind::AddArmour) => {
                    body_armour
                        .insert::<Armour, _>(BODYARMOUR_MORE_ARMOUR_RANGES.generate(ilevel, rng));
                }
                Some(BodyArmourAffixKind::AddLife) => {
                    body_armour
                        .insert::<MoreLife, _>(BODYARMOUR_MORE_LIFE_RANGES.generate(ilevel, rng));
                }
                None => {}
            }
        }
        body_armour.equipment_entity()
    }
}
