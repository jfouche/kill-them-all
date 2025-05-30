use super::{common::AffixProvider, Equipment};
use crate::components::{
    affix::{BaseArmour, IncreaseMovementSpeed, MoreArmour, MoreLife},
    item::{AffixConfigGenerator, ItemDescriptor, ItemRarity, ItemSpawnBundle},
    orb::OrbAction,
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Component)]
#[require(
    Name::new("Boots"),
    Equipment::Boots,
    MoreArmour,
    MoreLife,
    IncreaseMovementSpeed
)]
pub struct Boots {
    affix_provider: BootsAffixProvider,
    implicit_label: String,
}

impl ItemSpawnBundle for Boots {
    type Implicit = BaseArmour;

    fn new(ilevel: u16, rng: &mut ThreadRng) -> (Self, Self::Implicit) {
        let implicit = BaseArmour(rng.random_range(1..=4) as f32);
        let item = Boots {
            affix_provider: BootsAffixProvider::new(ilevel),
            implicit_label: implicit.to_string(),
        };
        (item, implicit)
    }
}

impl ItemDescriptor for Boots {
    fn title(&self) -> String {
        format!(
            "Boots (l{})\n{}",
            self.affix_provider.ilevel() + 1,
            self.implicit_label
        )
    }

    fn description(&self) -> String {
        self.affix_provider.item_description()
    }

    fn tile_index(&self, rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 63,
            ItemRarity::Magic => 65,
            ItemRarity::Rare => 66,
        }
    }
}

impl OrbAction for Boots {
    fn reset_affixes(&mut self, ecommands: &mut EntityCommands) {
        self.affix_provider.reset();
        ecommands.insert((MoreArmour(0.), MoreLife(0.), IncreaseMovementSpeed(0.)));
    }

    fn add_affixes(&mut self, ecommands: &mut EntityCommands, count: u16, rng: &mut ThreadRng) {
        let ilevel = self.affix_provider.ilevel();
        for _ in 0..count {
            match self.affix_provider.gen(rng) {
                Some(BootsAffixKind::MoreArmour) => {
                    let value_and_tier = MORE_ARMOUR_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<MoreArmour, _>(ecommands, value_and_tier);
                }
                Some(BootsAffixKind::AddLife) => {
                    let value_and_tier = MORE_LIFE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<MoreLife, _>(ecommands, value_and_tier);
                }
                Some(BootsAffixKind::IncreaseMovementSpeed) => {
                    let value_and_tier = INCR_MOVEMENT_SPEED_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<IncreaseMovementSpeed, _>(ecommands, value_and_tier);
                }
                None => {}
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum BootsAffixKind {
    AddLife,
    MoreArmour,
    IncreaseMovementSpeed,
}

const MORE_ARMOUR_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const MORE_LIFE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const INCR_MOVEMENT_SPEED_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

#[derive(Deref, DerefMut)]
struct BootsAffixProvider(AffixProvider<BootsAffixKind>);

impl BootsAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(
            BootsAffixKind::MoreArmour,
            MORE_ARMOUR_RANGES.weight(ilevel),
        );
        provider.add(BootsAffixKind::AddLife, MORE_LIFE_RANGES.weight(ilevel));
        provider.add(
            BootsAffixKind::IncreaseMovementSpeed,
            INCR_MOVEMENT_SPEED_RANGES.weight(ilevel),
        );
        BootsAffixProvider(AffixProvider::new::<Boots>(ilevel, provider))
    }
}
