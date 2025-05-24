use super::{common::AffixProvider, Equipment};
use crate::components::{
    affix::{BaseArmour, LifeRegen, MoreArmour, MoreLife},
    item::{AffixConfigGenerator, ItemDescriptor, ItemRarity, ItemSpawnBundle},
    orb::OrbAction,
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Component)]
#[require(
    Name::new("BodyArmour"),
    Equipment::BodyArmour,
    MoreArmour,
    MoreLife,
    LifeRegen
)]
pub struct BodyArmour {
    affix_provider: BodyArmourAffixProvider,
    implicit_label: String,
}

impl ItemSpawnBundle for BodyArmour {
    type Implicit = BaseArmour;

    fn new(ilevel: u16, rng: &mut ThreadRng) -> (Self, Self::Implicit) {
        let implicit = BaseArmour(rng.random_range(1..=4) as f32);
        let item = BodyArmour {
            affix_provider: BodyArmourAffixProvider::new(ilevel),
            implicit_label: implicit.to_string(),
        };
        (item, implicit)
    }
}

impl ItemDescriptor for BodyArmour {
    fn title(&self) -> String {
        format!(
            "Body armour ({})\n{}",
            self.affix_provider.ilevel(),
            self.implicit_label
        )
    }

    fn description(&self) -> String {
        self.affix_provider.item_description()
    }

    fn tile_index(&self, rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 0,
            ItemRarity::Magic => 2,
            ItemRarity::Rare => 3,
        }
    }
}

impl OrbAction for BodyArmour {
    fn reset_affixes(&mut self, ecommands: &mut EntityCommands) {
        self.affix_provider.reset();
        ecommands.insert((MoreArmour(0.), MoreLife(0.), LifeRegen(0.)));
    }

    fn add_affixes(&mut self, ecommands: &mut EntityCommands, count: u16, rng: &mut ThreadRng) {
        let ilevel = self.affix_provider.ilevel();
        for _ in 0..count {
            match self.affix_provider.gen(rng) {
                Some(BodyArmourAffixKind::MoreArmour) => {
                    let value_and_tier = MORE_ARMOUR_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<MoreArmour, _>(ecommands, value_and_tier);
                }
                Some(BodyArmourAffixKind::AddLife) => {
                    let value_and_tier = MORE_LIFE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<MoreLife, _>(ecommands, value_and_tier);
                }
                Some(BodyArmourAffixKind::LifeRegen) => {
                    let value_and_tier = LIFE_REGEN_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<LifeRegen, _>(ecommands, value_and_tier);
                }
                None => {}
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum BodyArmourAffixKind {
    AddLife,
    MoreArmour,
    LifeRegen,
}

const MORE_ARMOUR_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const MORE_LIFE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const LIFE_REGEN_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(1, (1, 2), 20), (7, (2, 8), 20), (19, (8, 16), 20)];

#[derive(Deref, DerefMut)]
struct BodyArmourAffixProvider(AffixProvider<BodyArmourAffixKind>);

impl BodyArmourAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(
            BodyArmourAffixKind::MoreArmour,
            MORE_ARMOUR_RANGES.weight(ilevel),
        );
        provider.add(
            BodyArmourAffixKind::AddLife,
            MORE_LIFE_RANGES.weight(ilevel),
        );
        provider.add(
            BodyArmourAffixKind::LifeRegen,
            LIFE_REGEN_RANGES.weight(ilevel),
        );
        BodyArmourAffixProvider(AffixProvider::new::<BodyArmour>(ilevel, provider))
    }
}
