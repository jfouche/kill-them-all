use super::rng_provider::{Generator, RngKindProvider};
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct Upgrades(pub Vec<Upgrade>);

pub trait ProvideUpgrades {
    fn armour(&self) -> f32;
    fn more_life(&self) -> f32;
    fn increase_max_life(&self) -> f32;
    fn life_regen(&self) -> f32;
    fn increase_movement_speed(&self) -> f32;
    fn increase_attack_speed(&self) -> f32;
    fn pierce_chance(&self) -> f32;
}

impl ProvideUpgrades for Upgrades {
    fn armour(&self) -> f32 {
        0.
    }

    fn more_life(&self) -> f32 {
        0.
    }

    fn increase_max_life(&self) -> f32 {
        self.0.iter().fold(0., |acc, u| {
            acc + match *u {
                Upgrade::IncreaseMaxLife(v) => v,
                _ => 0.,
            }
        })
    }

    fn life_regen(&self) -> f32 {
        self.0.iter().fold(0., |acc, u| {
            acc + match *u {
                Upgrade::IncreaseLifeRegen(v) => v,
                _ => 0.,
            }
        })
    }

    fn increase_movement_speed(&self) -> f32 {
        self.0.iter().fold(0., |acc, u| {
            acc + match *u {
                Upgrade::IncreasemovementSpeed(v) => v,
                _ => 0.,
            }
        })
    }

    fn increase_attack_speed(&self) -> f32 {
        self.0.iter().fold(0., |acc, u| {
            acc + match *u {
                Upgrade::IncreaseAttackSpeed(v) => v,
                _ => 0.,
            }
        })
    }

    fn pierce_chance(&self) -> f32 {
        self.0.iter().fold(0., |acc, u| {
            acc + match *u {
                Upgrade::PierceChance(v) => v,
                _ => 0.,
            }
        })
    }
}

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
            UpgradeKind::Pierce => Upgrade::PierceChance(rng.gen_range(2..20) as f32),
        }
    }
}

#[derive(Clone, Copy, Component, Reflect)]
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
    PierceChance(f32),
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
