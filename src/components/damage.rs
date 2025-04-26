use super::{
    affix::{IncreaseDamage, MoreDamage, PierceChance},
    character::Target,
    LifeTime, GROUP_ALL, GROUP_DAMAGER, GROUP_ENEMY, GROUP_ITEM, GROUP_PLAYER,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{rngs::ThreadRng, Rng};

///
/// Component which stores the base [HitDamageRange] of a [crate::components::equipment::Weapon]
/// which damage on hit.
///
#[derive(Component, Clone, Copy, Reflect)]
#[require(HitDamageRange)]
pub struct BaseHitDamageRange(pub HitDamageRange);

impl BaseHitDamageRange {
    pub fn new(min: f32, max: f32) -> Self {
        BaseHitDamageRange(HitDamageRange { min, max })
    }
}

///
/// Component which allows to generate hit [Damage] base on RNG
///
#[derive(Component, Default, Clone, Copy, Reflect)]
pub struct HitDamageRange {
    pub min: f32,
    pub max: f32,
}

impl HitDamageRange {
    pub fn new(min: f32, max: f32) -> Self {
        HitDamageRange { min, max }
    }

    pub fn gen(&self, rng: &mut ThreadRng) -> Damage {
        let damage = if self.min == self.max {
            self.min
        } else {
            rng.random_range(self.min..=self.max)
        };
        Damage(damage)
    }

    pub fn init(&mut self, base: &BaseHitDamageRange) {
        self.min = base.0.min;
        self.max = base.0.max;
    }

    pub fn add(&mut self, range: &HitDamageRange) {
        self.min += range.min;
        self.max += range.max;
    }

    pub fn more(&mut self, more: &MoreDamage) {
        self.min += more.0;
        self.max += more.0;
    }

    pub fn increase(&mut self, increase: &IncreaseDamage) {
        let multiplier = 1. + increase.0 / 100.;
        self.min *= multiplier;
        self.max *= multiplier;
    }
}

///
/// Base damage over time
///
#[derive(Component, Default, Clone, Copy, Deref, Reflect)]
#[require(DamageOverTime)]
pub struct BaseDamageOverTime(pub f32);

impl BaseDamageOverTime {
    pub fn damage_over_time(&self, more: &MoreDamage, increase: &IncreaseDamage) -> DamageOverTime {
        DamageOverTime((self.0 + **more) * (1. + **increase / 100.))
    }
}

///
/// Damage over time
///
#[derive(Component, Default, Clone, Copy, Deref, Reflect)]
pub struct DamageOverTime(pub f32);

impl DamageOverTime {
    pub fn damage(&self, time: &Time) -> Damage {
        Damage(self.0 * time.delta_secs())
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

/// A [Damager] is an entity which damages a [crate::components::character::Character].
///
/// It requires:
/// - At least one of [HitDamageRange] or [DamageOverTime]
/// - A [Collider]
/// - A [CollisionGroups], initialized by [Damager::collision_groups]
#[derive(Component, Default)]
#[require(
    Transform,
    RigidBody,
    Collider,
    CollisionGroups = Damager::collision_groups(Target::Monster),
    Sensor,
    ActiveEvents::COLLISION_EVENTS
)]
pub struct Damager;

impl Damager {
    pub fn collision_groups(target: Target) -> CollisionGroups {
        let filter = match target {
            Target::Player => GROUP_ITEM | GROUP_ENEMY,
            Target::Monster => GROUP_ITEM | GROUP_PLAYER,
        };
        CollisionGroups::new(GROUP_DAMAGER, GROUP_ALL & !filter)
    }
}

/// Helper to spawn required [Damager] dynamic components
#[derive(Bundle)]
pub struct DamagerParams {
    pub transform: Transform,
    pub collision_groups: CollisionGroups,
}

/// A [Projectile] is an [Damager] which is sent, and can pierce
#[derive(Component, Default)]
#[require(Damager, LifeTime::new(5.), PierceChance, Velocity)]
pub struct Projectile;

/// Helper to spawn required [Projectile] dynamic components
#[derive(Bundle)]
pub struct ProjectileParams {
    pub pierce_chance: PierceChance,
    pub velocity: Velocity,
}
