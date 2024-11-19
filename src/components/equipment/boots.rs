use super::*;
use crate::components::{rng_provider::*, *};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BootsAffixKind {
    AddLife,
    AddArmour,
    IncreaseMovementSpeed,
}

#[derive(Deref, DerefMut)]
pub struct BootsAffixProvider(RngKindProvider<BootsAffixKind>);

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
pub struct Boots;

impl Boots {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntity {
        let mut provider = BootsAffixProvider::new();
        let rarity = EquipmentRarityProvider::new()
            .gen(rng)
            .expect("At least one rarity");

        let mut boots_commands = commands.spawn((Boots, Name::new("Boots")));

        let mut labels = vec![];
        for _ in 0..rarity.n_affix() {
            match provider.gen(rng) {
                Some(BootsAffixKind::AddArmour) => {
                    let affix = Armour(rng.gen_range(1. ..=3.));
                    labels.push(affix.to_string());
                    boots_commands.insert(affix);
                }
                Some(BootsAffixKind::AddLife) => {
                    let affix = MoreLife(rng.gen_range(5. ..=10.));
                    labels.push(affix.to_string());
                    boots_commands.insert(affix);
                }
                Some(BootsAffixKind::IncreaseMovementSpeed) => {
                    let affix = IncreaseMovementSpeed(rng.gen_range(5. ..=30.));
                    labels.push(affix.to_string());
                    boots_commands.insert(affix);
                }
                None => {}
            }
        }
        boots_commands.insert(AffixesLabels(labels.join("\n")));

        let tile_index = match rarity {
            EquipmentRarity::Normal => 63,
            EquipmentRarity::Magic => 65,
            EquipmentRarity::Rare => 66,
        };

        EquipmentEntity {
            entity: boots_commands.id(),
            tile_index,
            label: labels.join("\n"),
        }
    }
}
