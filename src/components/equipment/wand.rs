use super::{
    common::AffixProvider,
    weapon::{BaseAttackSpeed, Weapon},
};
use crate::components::{
    affix::{IncreaseAttackSpeed, IncreaseDamage, MoreDamage, PierceChance},
    damage::BaseHitDamageRange,
    item::{AffixConfigGenerator, ItemDescriptor, ItemRarity, ItemSpawnConfig},
    orb::OrbAction,
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

/// A [Wand]
#[derive(Component)]
#[require(
    Name::new("Wand"),
    Weapon,
    BaseHitDamageRange::new(1., 2.),
    BaseAttackSpeed,
    MoreDamage,
    IncreaseDamage,
    PierceChance,
    IncreaseAttackSpeed
)]
pub struct Wand {
    affix_provider: WandAffixProvider,
    implicit: String,
}

impl ItemSpawnConfig for Wand {
    type Implicit = BaseAttackSpeed;

    fn new(ilevel: u16) -> Self {
        Wand {
            affix_provider: WandAffixProvider::new(ilevel),
            implicit: "".into(),
        }
    }

    fn implicit(&mut self, rng: &mut ThreadRng) -> Self::Implicit {
        let implicit = BaseAttackSpeed(rng.random_range(1.0..1.5));
        self.implicit = implicit.to_string();
        implicit
    }
}

impl ItemDescriptor for Wand {
    fn title(&self) -> String {
        format!(
            "Wand (l{})\n{}",
            self.affix_provider.ilevel() + 1,
            self.implicit
        )
    }

    fn description(&self) -> String {
        self.affix_provider.item_description()
    }

    fn tile_index(&self, rarity: ItemRarity) -> usize {
        match rarity {
            ItemRarity::Normal => 318,
            ItemRarity::Magic => 320,
            ItemRarity::Rare => 321,
        }
    }
}

impl OrbAction for Wand {
    fn reset_affixes(&mut self, ecommands: &mut EntityCommands) {
        self.affix_provider.reset();
        ecommands.insert((
            MoreDamage(0.),
            IncreaseDamage(0.),
            PierceChance(0.),
            IncreaseAttackSpeed(0.),
        ));
    }

    fn add_affixes(&mut self, ecommands: &mut EntityCommands, count: u16, rng: &mut ThreadRng) {
        let ilevel = self.affix_provider.ilevel();
        for _ in 0..count {
            match self.affix_provider.gen(rng) {
                Some(WandAffixKind::MoreDamage) => {
                    let value_and_tier = MORE_DAMAGE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<MoreDamage, _>(ecommands, value_and_tier);
                }
                Some(WandAffixKind::IncreaseDamage) => {
                    let value_and_tier = INCR_DAMAGE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<IncreaseDamage, _>(ecommands, value_and_tier);
                }
                Some(WandAffixKind::PierceChance) => {
                    let value_and_tier = PIERCE_CHANCE_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<PierceChance, _>(ecommands, value_and_tier);
                }
                Some(WandAffixKind::IncreaseAttackSpeed) => {
                    let value_and_tier = INCR_ATTACK_SPEED_RANGES.generate(ilevel, rng);
                    self.affix_provider
                        .set::<IncreaseAttackSpeed, _>(ecommands, value_and_tier);
                }
                None => {}
            }
        }
    }
}

/// All [Wand] available affixes
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum WandAffixKind {
    MoreDamage,
    IncreaseDamage,
    PierceChance,
    IncreaseAttackSpeed,
}

const MORE_DAMAGE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const INCR_DAMAGE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 20), (10, (10, 24), 20), (17, (25, 29), 20)];

const PIERCE_CHANCE_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 10), (10, (10, 24), 10), (17, (25, 29), 10)];

const INCR_ATTACK_SPEED_RANGES: &[(u16, (u16, u16), usize); 3] =
    &[(4, (3, 9), 10), (10, (10, 24), 10), (17, (25, 29), 10)];

#[derive(Deref, DerefMut)]
struct WandAffixProvider(AffixProvider<WandAffixKind>);

impl WandAffixProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(WandAffixKind::MoreDamage, MORE_DAMAGE_RANGES.weight(ilevel));
        provider.add(
            WandAffixKind::IncreaseDamage,
            INCR_DAMAGE_RANGES.weight(ilevel),
        );
        provider.add(
            WandAffixKind::PierceChance,
            PIERCE_CHANCE_RANGES.weight(ilevel),
        );
        provider.add(
            WandAffixKind::IncreaseAttackSpeed,
            INCR_ATTACK_SPEED_RANGES.weight(ilevel),
        );
        WandAffixProvider(AffixProvider::new::<Wand>(ilevel, provider))
    }
}
