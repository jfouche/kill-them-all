use crate::components::{rng_provider::RngKindProvider, *};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AmuletAffixKind {
    AddLife,
    AddArmour,
}

#[derive(Deref, DerefMut)]
pub struct AmuletAffixProvider(RngKindProvider<AmuletAffixKind>);

impl AmuletAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(AmuletAffixKind::AddArmour, 20);
        provider.add(AmuletAffixKind::AddLife, 20);
        AmuletAffixProvider(provider)
    }
}

#[derive(Component)]
pub struct Amulet;

impl Amulet {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntity {
        let mut provider = AmuletAffixProvider::new();
        let rarity = EquipmentRarityProvider::new()
            .gen(rng)
            .expect("At least one rarity");

        let mut amulet_commands = commands.spawn((Amulet, Name::new("Amulet")));

        let mut labels = vec![];
        for _ in 0..rarity.n_affix() {
            match provider.gen(rng) {
                Some(AmuletAffixKind::AddArmour) => {
                    let affix = Armour(rng.gen_range(1. ..=3.));
                    labels.push(affix.label());
                    amulet_commands.insert(affix);
                }
                Some(AmuletAffixKind::AddLife) => {
                    let affix = MoreLife(rng.gen_range(5. ..=10.));
                    labels.push(affix.label());
                    amulet_commands.insert(affix);
                }
                None => {}
            }
        }
        amulet_commands.insert(AffixesLabels(labels.join("\n")));

        let tile_index = match rarity {
            EquipmentRarity::Normal => 213,
            EquipmentRarity::Magic => 215,
            EquipmentRarity::Rare => 216,
        };

        EquipmentEntity {
            entity: amulet_commands.id(),
            tile_index,
            label: labels.join("\n"),
        }
    }
}
