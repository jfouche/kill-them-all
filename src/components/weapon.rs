use super::*;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};
use std::time::Duration;

///
/// A [Weapon] should be a child of a [Character] to make attacks
///
#[derive(Component, Default)]
#[require(
    BaseDamageRange(|| BaseDamageRange::new(1., 2.)),
    BaseAttackSpeed,
    AttackTimer
)]
pub struct Weapon;

///
/// Component which stores the base [DamageRange] of a [Weapon]
///
#[derive(Component, Clone, Copy, Reflect)]
#[require(DamageRange)]
pub struct BaseDamageRange(pub DamageRange);

impl BaseDamageRange {
    pub fn new(min: f32, max: f32) -> Self {
        BaseDamageRange(DamageRange { min, max })
    }
}

///
/// Component which allows to generate [Damage] base on RNG
///
#[derive(Component, Clone, Copy, Reflect)]
pub struct DamageRange {
    pub min: f32,
    pub max: f32,
}

impl Default for DamageRange {
    fn default() -> Self {
        DamageRange { min: 1., max: 2. }
    }
}

impl DamageRange {
    pub fn new(min: f32, max: f32) -> Self {
        DamageRange { min, max }
    }

    pub fn gen(&self, rng: &mut ThreadRng) -> Damage {
        let damage = rng.gen_range(self.min..=self.max);
        Damage(damage)
    }
}

impl std::ops::Add<&MoreDamage> for &BaseDamageRange {
    type Output = DamageRange;
    fn add(self, more: &MoreDamage) -> Self::Output {
        DamageRange {
            min: self.0.min + **more,
            max: self.0.max + **more,
        }
    }
}

impl std::ops::Mul<&IncreaseDamage> for DamageRange {
    type Output = DamageRange;
    fn mul(self, increase: &IncreaseDamage) -> Self::Output {
        let multiplier = 1. + **increase / 100.;
        DamageRange {
            min: self.min * multiplier,
            max: self.max * multiplier,
        }
    }
}

///
/// Damage
///
#[derive(Clone, Copy, Component, Default, Deref, Reflect)]
pub struct Damage(pub f32);

impl std::ops::AddAssign for Damage {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl std::ops::Sub<f32> for Damage {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        let damage = (self.0 - rhs).max(0.);
        Damage(damage)
    }
}

///
/// It represents the base attack per second
///
#[derive(Component, Default, Clone, Copy, Deref, Reflect)]
#[require(AttackSpeed, AttackTimer)]
pub struct BaseAttackSpeed(pub f32);

impl std::ops::Mul<&IncreaseAttackSpeed> for &BaseAttackSpeed {
    type Output = AttackSpeed;
    fn mul(self, rhs: &IncreaseAttackSpeed) -> Self::Output {
        AttackSpeed(self.0 * (1.0 + rhs.0 / 100.))
    }
}

/// Attack per second
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Reflect)]
pub struct AttackSpeed(pub f32);

#[derive(Component, Deref, DerefMut, Reflect)]
pub struct AttackTimer(pub Timer);

impl Default for AttackTimer {
    fn default() -> Self {
        AttackTimer(Timer::from_seconds(1., TimerMode::Repeating))
    }
}

impl AttackTimer {
    pub fn set_attack_speed(&mut self, attack_speed: AttackSpeed) {
        self.set_duration(Duration::from_secs_f32(1. / *attack_speed));
    }
}

/// [Ammo]'s components required:
/// - [DamageRange]
/// - [Collider]
#[derive(Component, Default)]
#[require(
    DamageRange,
    LifeTime(|| LifeTime::new(5.)),
    RigidBody,
    Collider,
    Sensor,
    CollisionGroups(Ammo::player_collision_groups),
    ActiveEvents(|| ActiveEvents::COLLISION_EVENTS)
)]
pub struct Ammo;

impl Ammo {
    fn player_collision_groups() -> CollisionGroups {
        CollisionGroups::new(GROUP_BULLET, Group::ALL & !(GROUP_BONUS | GROUP_PLAYER))
    }

    fn monster_collision_groups() -> CollisionGroups {
        CollisionGroups::new(GROUP_BULLET, Group::ALL & !(GROUP_BONUS | GROUP_ENEMY))
    }
}

#[derive(Component, Default)]
#[require(Ammo, PierceChance, Velocity)]
pub struct Projectile;
