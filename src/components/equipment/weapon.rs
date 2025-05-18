use super::Equipment;
use crate::components::affix::IncreaseAttackSpeed;
use bevy::prelude::*;
use std::time::Duration;

/// A [Weapon] should be a child of a [crate::components::character::Character] in
/// order to be active
#[derive(Component, Default)]
#[require(Equipment::Weapon)]
pub struct Weapon;

///
/// It represents the [Weapon] base attack per second
///
#[derive(Component, Default, Clone, Copy, Deref, Reflect)]
#[require(AttackSpeed, AttackTimer)]
pub struct BaseAttackSpeed(pub f32);

impl std::fmt::Display for BaseAttackSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Base attack speed: {}", self.0)
    }
}

/// Attack per second
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Reflect)]
pub struct AttackSpeed(pub f32);

impl AttackSpeed {
    pub fn init(&mut self, base: &BaseAttackSpeed) {
        self.0 = base.0;
    }

    pub fn increase(&mut self, increase: &IncreaseAttackSpeed) {
        self.0 *= 1. + **increase / 100.;
    }
}

#[derive(Component, Deref, DerefMut, Reflect)]
pub struct AttackTimer(pub Timer);

impl Default for AttackTimer {
    fn default() -> Self {
        AttackTimer(Timer::from_seconds(1., TimerMode::Once))
    }
}

impl AttackTimer {
    pub fn set_attack_speed(&mut self, attack_speed: AttackSpeed) {
        if *attack_speed <= 0.0 {
            warn!("AttackSpeed is <= 0.0");
            return;
        }
        self.set_duration(Duration::from_secs_f32(1. / *attack_speed));
    }
}
