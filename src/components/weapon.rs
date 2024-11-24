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
    pub fn new(attack_speed: f32) -> Self {
        AttackTimer(Timer::from_seconds(1. / attack_speed, TimerMode::Repeating))
    }

    pub fn set_attack_speed(&mut self, attack_speed: AttackSpeed) {
        self.set_duration(Duration::from_secs_f32(1. / *attack_speed));
    }
}

#[derive(Component)]
pub struct Ammo;

#[derive(Bundle, Default)]
pub struct AmmoConfig {
    pub damage: Damage,
    pub pierce: PierceChance,
    pub velocity: Velocity,
    pub collider: Collider,
}

#[derive(Bundle)]
pub struct AmmoBundle {
    tag: Ammo,
    config: AmmoConfig,
    lifetime: LifeTime,
    body: RigidBody,
    // mass: ColliderMassProperties,
    sensor: Sensor,
    collision_groups: CollisionGroups,
    locked_axes: LockedAxes,
    active_events: ActiveEvents,
}

impl Default for AmmoBundle {
    fn default() -> Self {
        AmmoBundle {
            tag: Ammo,
            config: AmmoConfig::default(),
            lifetime: LifeTime::new(3.),
            body: RigidBody::Dynamic,
            // mass: ColliderMassProperties::MassProperties(MassProperties {
            //     mass: 0.001,
            //     principal_inertia: 0.001,
            //     ..Default::default()
            // }),
            sensor: Sensor,
            collision_groups: CollisionGroups::new(
                GROUP_BULLET,
                Group::ALL & !(GROUP_BONUS | GROUP_PLAYER),
            ),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

impl AmmoBundle {
    pub fn new(config: AmmoConfig) -> Self {
        AmmoBundle {
            config,
            ..Default::default()
        }
    }
}
