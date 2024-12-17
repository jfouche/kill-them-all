use super::*;
use crate::components::{rng_provider::*, *};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BodyArmourAffixKind {
    AddLife,
    AddArmour,
}

#[derive(Deref, DerefMut)]
pub struct BodyArmourAffixProvider(RngKindProvider<BodyArmourAffixKind>);

impl BodyArmourAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(BodyArmourAffixKind::AddArmour, 20);
        provider.add(BodyArmourAffixKind::AddLife, 20);
        BodyArmourAffixProvider(provider)
    }
}

#[derive(Component)]
pub struct BodyArmour;

impl BodyArmour {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntity {
        let mut provider = BodyArmourAffixProvider::new();
        let rarity = EquipmentRarityProvider::new()
            .gen(rng)
            .expect("At least one rarity");
        let tile_index = match rarity {
            EquipmentRarity::Normal => 0,
            EquipmentRarity::Magic => 2,
            EquipmentRarity::Rare => 3,
        };
        let mut body_armour_commands =
            commands.spawn((BodyArmour, Name::new("BodyArmour"), TileIndex(tile_index)));

        let mut labels = vec![];
        for _ in 0..rarity.n_affix() {
            match provider.gen(rng) {
                Some(BodyArmourAffixKind::AddArmour) => {
                    let affix = Armour(rng.gen_range(1..=3) as f32);
                    labels.push(affix.to_string());
                    body_armour_commands.insert(affix);
                }
                Some(BodyArmourAffixKind::AddLife) => {
                    let affix = MoreLife(rng.gen_range(5..=10) as f32);
                    labels.push(affix.to_string());
                    body_armour_commands.insert(affix);
                }
                None => {}
            }
        }
        body_armour_commands.insert(AffixesLabels(labels.join("\n")));

        EquipmentEntity {
            entity: body_armour_commands.id(),
            tile_index,
            label: labels.join("\n"),
        }
    }
}
