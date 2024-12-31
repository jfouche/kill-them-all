use crate::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{rngs::ThreadRng, Rng};
use std::time::Duration;

///
/// A [Weapon] should be a child of a [Character]
///
#[derive(Component, Default)]
#[require(
    Equipment(|| Equipment::Weapon)
)]
pub struct Weapon;

///
/// Component which stores the base [HitDamageRange] of a [Weapon]
/// which damage on hit.
///
#[derive(Component, Clone, Copy, Reflect)]
#[require(HitDamageRange)]
pub struct BaseHitDamageRange(pub HitDamageRange);

impl BaseHitDamageRange {
    pub fn new(min: f32, max: f32) -> Self {
        BaseHitDamageRange(HitDamageRange { min, max })
    }

    /// Return the real [HitDamageRange] after applying [MoreDamage] and [IncreaseDamage]
    pub fn damage_range(&self, more: &MoreDamage, increase: &IncreaseDamage) -> HitDamageRange {
        let multiplier = 1. + **increase / 100.;
        HitDamageRange {
            min: (self.0.min + **more) * multiplier,
            max: (self.0.max + **more) * multiplier,
        }
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
            rng.gen_range(self.min..=self.max)
        };
        Damage(damage)
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

///
/// It represents the [Weapon] base attack per second
///
#[derive(Component, Default, Clone, Copy, Deref, Reflect)]
#[require(AttackSpeed, AttackTimer)]
pub struct BaseAttackSpeed(pub f32);

impl BaseAttackSpeed {
    pub fn attack_speed(&self, increase: &IncreaseAttackSpeed) -> AttackSpeed {
        AttackSpeed(self.0 * (1. + increase.0 / 100.))
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
        if *attack_speed == 0.0 {
            warn!("AttackSpeed is 0.");
            return;
        }
        self.set_duration(Duration::from_secs_f32(1. / *attack_speed));
    }
}

/// A [Damager] is an entity which damages a [Character].
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
    CollisionGroups(|| Damager::collision_groups(Target::Monster)),
    Sensor,
    ActiveEvents(|| ActiveEvents::COLLISION_EVENTS)
)]
pub struct Damager;

impl Damager {
    pub fn collision_groups(target: Target) -> CollisionGroups {
        let filter = match target {
            Target::Player => GROUP_BONUS | GROUP_ENEMY,
            Target::Monster => GROUP_BONUS | GROUP_PLAYER,
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
#[require(
    Damager,
    LifeTime(|| LifeTime::new(5.)),
    PierceChance,
    Velocity
)]
pub struct Projectile;

/// Helper to spawn required [Projectile] dynamic components
#[derive(Bundle)]
pub struct ProjectileParams {
    pub pierce_chance: PierceChance,
    pub velocity: Velocity,
}
