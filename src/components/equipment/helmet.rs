use super::*;
use crate::components::{rng_provider::*, *};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum HelmetAffixKind {
    AddLife,
    AddArmour,
}

#[derive(Deref, DerefMut)]
pub struct HelmetAffixProvider(RngKindProvider<HelmetAffixKind>);

impl HelmetAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(HelmetAffixKind::AddArmour, 20);
        provider.add(HelmetAffixKind::AddLife, 20);
        HelmetAffixProvider(provider)
    }
}

#[derive(Component)]
pub struct Helmet;

impl Helmet {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntity {
        let mut provider = HelmetAffixProvider::new();
        let rarity = EquipmentRarityProvider::new()
            .gen(rng)
            .expect("At least one rarity");
        let tile_index = match rarity {
            EquipmentRarity::Normal => 182,
            EquipmentRarity::Magic => 184,
            EquipmentRarity::Rare => 185,
        };
        let mut helmet_commands =
            commands.spawn((Helmet, Name::new("Helmet"), TileIndex(tile_index)));

        let mut labels = vec![];
        for _ in 0..rarity.n_affix() {
            match provider.gen(rng) {
                Some(HelmetAffixKind::AddArmour) => {
                    let affix = Armour(rng.gen_range(1. ..=3.));
                    labels.push(affix.to_string());
                    helmet_commands.insert(affix);
                }
                Some(HelmetAffixKind::AddLife) => {
                    let affix = MoreLife(rng.gen_range(5. ..=10.));
                    labels.push(affix.to_string());
                    helmet_commands.insert(affix);
                }
                None => {}
            }
        }
        helmet_commands.insert(AffixesLabels(labels.join("\n")));

        EquipmentEntity {
            entity: helmet_commands.id(),
            tile_index,
            label: labels.join("\n"),
        }
    }
}
