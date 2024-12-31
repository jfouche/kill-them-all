use super::*;
use crate::components::{rng_provider::*, *};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum BodyArmourAffixKind {
    AddLife,
    AddArmour,
}

#[derive(Deref, DerefMut)]
struct BodyArmourAffixProvider(RngKindProvider<BodyArmourAffixKind>);

impl BodyArmourAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(BodyArmourAffixKind::AddArmour, 20);
        provider.add(BodyArmourAffixKind::AddLife, 20);
        BodyArmourAffixProvider(provider)
    }
}

#[derive(Component)]
#[require(
    Name(|| Name::new("BodyArmour")),
    Equipment(|| Equipment::BodyArmour)
)]
pub struct BodyArmour;

impl EquipmentUI for BodyArmour {
    fn title() -> String {
        "Body armour".into()
    }
    fn tile_index(rarity: EquipmentRarity) -> usize {
        match rarity {
            EquipmentRarity::Normal => 0,
            EquipmentRarity::Magic => 2,
            EquipmentRarity::Rare => 3,
        }
    }
}

impl BodyArmour {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntityInfo {
        let mut provider = BodyArmourAffixProvider::new();
        let mut body_armour = AffixesInserter::spawn(commands, BodyArmour, rng);
        for _ in 0..body_armour.n_affix() {
            match provider.gen(rng) {
                Some(BodyArmourAffixKind::AddArmour) => {
                    body_armour.insert::<Armour, u16>(rng.gen_range(1..=3));
                }
                Some(BodyArmourAffixKind::AddLife) => {
                    body_armour.insert::<MoreLife, u16>(rng.gen_range(5..=10));
                }
                None => {}
            }
        }
        body_armour.equipment_entity()
    }
}
