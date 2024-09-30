use super::rng_provider::{Generator, RngKindProvider};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Component, Default, Deref, DerefMut)]
pub struct Upgrades(pub Vec<Upgrade>);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpgradeKind {
    IncreaseMaxLife,
    IncreaseLifeRegen,
    IncreaseAttackSpeed,
    IncreasemovementSpeed,
    Pierce,
}

impl Generator<Upgrade> for UpgradeKind {
    fn generate(&self, rng: &mut ThreadRng) -> Upgrade {
        match self {
            UpgradeKind::IncreaseMaxLife => Upgrade::IncreaseMaxLife(rng.gen_range(2..10) as f32),
            UpgradeKind::IncreaseLifeRegen => {
                Upgrade::IncreaseLifeRegen(rng.gen_range(2..10) as f32)
            }
            UpgradeKind::IncreaseAttackSpeed => {
                Upgrade::IncreaseAttackSpeed(rng.gen_range(2..20) as f32)
            }
            UpgradeKind::IncreasemovementSpeed => {
                Upgrade::IncreasemovementSpeed(rng.gen_range(2..20) as f32)
            }
            UpgradeKind::Pierce => Upgrade::Pierce(rng.gen_range(2..20) as f32),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Upgrade {
    /// Increase max life percentage, 1.0 is 100%
    IncreaseMaxLife(f32),
    /// Increase life regen percentage, 1.0 is 100%
    IncreaseLifeRegen(f32),
    /// Increase attack speed percentage, 1.0 is 100%
    IncreaseAttackSpeed(f32),
    /// Increase movement speed percentage, 1.0 is 100%
    IncreasemovementSpeed(f32),
    // Pierce allow to not despawn when hitting
    Pierce(f32),
}

#[derive(Deref, DerefMut)]
pub struct UpgradeProvider(RngKindProvider<UpgradeKind, Upgrade>);

impl UpgradeProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::<UpgradeKind, Upgrade>::default();
        provider.add(UpgradeKind::IncreaseMaxLife, 40);
        provider.add(UpgradeKind::IncreaseLifeRegen, 40);
        provider.add(UpgradeKind::IncreaseAttackSpeed, 20);
        provider.add(UpgradeKind::IncreasemovementSpeed, 40);
        provider.add(UpgradeKind::Pierce, 20);

        UpgradeProvider(provider)
    }
}
