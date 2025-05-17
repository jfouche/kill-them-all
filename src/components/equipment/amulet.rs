use super::{common::AffixProvider, Equipment};
use crate::components::{
    affix::{Armour, MoreLife, PierceChance},
    item::{AffixConfigGenerator, ItemDescriptor, ItemRarity},
    orb::OrbAction,
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Component)]
#[require(Name::new("Amulet"), Equipment::Amulet, Armour, MoreLife, PierceChance)]
pub struct Amulet {
    affix_provider: AmuletAffixProvider,
}

impl Amulet {
    pub fn new(ilevel: u16) -> Self {
        Amulet {
            affix_provider: AmuletAffixProvider::new(ilevel),
        }
    }
}

impl ItemDescriptor for Amulet {
    fn title(&self) -> String {
        format!("Amulet (l{})", self.affix_provider.ilevel() + 1)
    }

    fn description(&self) -> String {
        self.affix_provider.item_description()
    }

    fn tile_index(&self, rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 213,
            ItemRarity::Magic => 215,
            ItemRarity::Rare => 216,
        }
    }
}

impl OrbAction for Amulet {
    fn reset_affixes(&mut self, ecommands: &mut EntityCommands) {
        self.affix_provider.reset();
        ecommands.insert((Armour(0.), MoreLife(0.), PierceChance(0.)));
    }

    fn add_affixes(&mut self, ecommands: &mut EntityCommands, count: u16, rng: &mut ThreadRng) {
        let ilevel = self.affix_provider.ilevel();
        for _ in 0..count {
            match self.affix_provider.gen(rng) {
                Some(AmuletAffixKind::AddArmour) => {
                    let value_and_tier = MORE_ARMOUR_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<Armour, _>(ecommands, value_and_tier);
                }
                Some(AmuletAffixKind::MoreLife) => {
                    let value_and_tier = MORE_LIFE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<MoreLife, _>(ecommands, value_and_tier);
                }
                Some(AmuletAffixKind::PierceChance) => {
                    let value_and_tier = PIERCE_CHANCE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<PierceChance, _>(ecommands, value_and_tier);
                }
                None => {}
            }
        }
    }
}

/// All available affixes for [Amulet]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum AmuletAffixKind {
    MoreLife,
    AddArmour,
    PierceChance,
}

const MORE_ARMOUR_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const MORE_LIFE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const PIERCE_CHANCE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 10), (10, (10, 24), 10), (17, (25, 29), 10)];

#[derive(Deref, DerefMut)]
struct AmuletAffixProvider(AffixProvider<AmuletAffixKind>);

impl AmuletAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();

        provider.add(
            AmuletAffixKind::AddArmour,
            MORE_ARMOUR_RANGES.weight(ilevel),
        );
        provider.add(AmuletAffixKind::MoreLife, MORE_LIFE_RANGES.weight(ilevel));
        provider.add(
            AmuletAffixKind::PierceChance,
            PIERCE_CHANCE_RANGES.weight(ilevel),
        );

        AmuletAffixProvider(AffixProvider::new::<Amulet>(ilevel, provider))
    }
}
