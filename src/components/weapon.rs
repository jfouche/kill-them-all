use super::*;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};
use std::{ops::RangeInclusive, time::Duration};

#[derive(Clone, Copy, Component, Default, Deref, Reflect)]
pub struct Damage(pub f32);

impl std::ops::Sub<f32> for Damage {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        let damage = (self.0 - rhs).max(0.);
        Damage(damage)
    }
}

/// Attack per second
#[derive(Component, Reflect)]
pub struct BaseAttackSpeed(pub f32);

impl std::ops::Mul<&IncreaseAttackSpeed> for &BaseAttackSpeed {
    type Output = AttackSpeed;
    fn mul(self, rhs: &IncreaseAttackSpeed) -> Self::Output {
        AttackSpeed(self.0 * (1.0 + rhs.0 / 100.))
    }
}

/// Attack per second
#[derive(Component, Default, Deref, Reflect)]
pub struct AttackSpeed(pub f32);

#[derive(Component, Deref, Reflect)]
pub struct DamageRange(pub RangeInclusive<f32>);

impl DamageRange {
    pub fn gen(&self, rng: &mut ThreadRng) -> Damage {
        let damage = rng.gen_range(self.0.clone());
        Damage(damage)
    }
}

#[derive(Component)]
pub struct Weapon;

#[derive(Component, Deref, DerefMut, Reflect)]
pub struct AttackTimer(pub Timer);

impl AttackTimer {
    pub fn set_attack_speed(&mut self, attack_speed: AttackSpeed) {
        self.set_duration(Duration::from_secs_f32(1. / *attack_speed));
    }
}
