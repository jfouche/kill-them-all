use super::{common::AffixProvider, Equipment};
use crate::components::{
    affix::{BaseArmour, LifeRegen, MoreArmour, MoreLife},
    item::{AffixConfigGenerator, ItemDescriptor, ItemRarity, ItemSpawnConfig},
    orb::OrbAction,
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Component)]
#[require(
    Name::new("Helmet"),
    Equipment::Helmet,
    MoreArmour,
    MoreLife,
    LifeRegen
)]
pub struct Helmet {
    affix_provider: HelmetAffixProvider,
    implicit: String,
}

impl ItemSpawnConfig for Helmet {
    type Implicit = BaseArmour;
    fn new(ilevel: u16) -> Self {
        Helmet {
            affix_provider: HelmetAffixProvider::new(ilevel),
            implicit: "".into(),
        }
    }

    fn implicit(&mut self, rng: &mut ThreadRng) -> Self::Implicit {
        let implicit = BaseArmour(rng.random_range(1..=4) as f32);
        self.implicit = implicit.to_string();
        implicit
    }
}

impl ItemDescriptor for Helmet {
    fn title(&self) -> String {
        format!(
            "Helmet (l{})\n{}",
            self.affix_provider.ilevel() + 1,
            self.implicit
        )
    }

    fn description(&self) -> String {
        self.affix_provider.item_description()
    }

    fn tile_index(&self, rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 182,
            ItemRarity::Magic => 184,
            ItemRarity::Rare => 185,
        }
    }
}

impl OrbAction for Helmet {
    fn reset_affixes(&mut self, ecommands: &mut EntityCommands) {
        self.affix_provider.reset();
        ecommands.insert((MoreArmour(0.), MoreLife(0.), LifeRegen(0.)));
    }

    fn add_affixes(&mut self, ecommands: &mut EntityCommands, count: u16, rng: &mut ThreadRng) {
        let ilevel = self.affix_provider.ilevel();
        for _ in 0..count {
            match self.affix_provider.gen(rng) {
                Some(HelmetAffixKind::MoreArmour) => {
                    let value_and_tier = MORE_ARMOUR_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<MoreArmour, _>(ecommands, value_and_tier);
                }
                Some(HelmetAffixKind::MoreLife) => {
                    let value_and_tier = MORE_LIFE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<MoreLife, _>(ecommands, value_and_tier);
                }
                Some(HelmetAffixKind::LifeRegen) => {
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
enum HelmetAffixKind {
    MoreLife,
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
struct HelmetAffixProvider(AffixProvider<HelmetAffixKind>);

impl HelmetAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(
            HelmetAffixKind::MoreArmour,
            MORE_ARMOUR_RANGES.weight(ilevel),
        );
        provider.add(HelmetAffixKind::MoreLife, MORE_LIFE_RANGES.weight(ilevel));
        provider.add(HelmetAffixKind::LifeRegen, LIFE_REGEN_RANGES.weight(ilevel));
        HelmetAffixProvider(AffixProvider::new::<Helmet>(ilevel, provider))
    }
}
