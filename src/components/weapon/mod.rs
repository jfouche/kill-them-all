mod gun;
pub use gun::*;

mod shuriken;
pub use shuriken::*;

use super::*;
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

#[derive(Clone, Copy, Component, Default, Deref)]
pub struct Damage(pub f32);

impl std::ops::Sub<f32> for Damage {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        let damage = (self.0 - rhs).max(0.);
        Damage(damage)
    }
}

pub enum WeaponType {
    Gun,
    _Shuriken,
}

#[derive(Component)]
pub struct Weapon {
    _weapon_type: WeaponType,
    /// Attack per second
    attack_speed: f32,
    damage_min: f32,
    damage_max: f32,
    timer: Timer,
    ready: bool,
}

impl From<WeaponType> for Weapon {
    fn from(value: WeaponType) -> Self {
        match value {
            WeaponType::Gun => gun(),
            WeaponType::_Shuriken => shuriken(),
        }
    }
}

impl Weapon {
    fn new(
        weapon_type: WeaponType,
        attack_per_second: f32,
        damage_min: f32,
        damage_max: f32,
    ) -> Self {
        Weapon {
            _weapon_type: weapon_type,
            attack_speed: attack_per_second,
            damage_min,
            damage_max,
            timer: Timer::from_seconds(1. / attack_per_second, TimerMode::Repeating),
            ready: false,
        }
    }

    pub fn attack(&mut self) -> Damage {
        self.ready = false;
        let damage = rand::thread_rng().gen_range(self.damage_min..=self.damage_max);
        Damage(damage)
    }

    pub fn tick(&mut self, delta: Duration, player_attack_speed_increases: f32) -> &Timer {
        let attack_speed = self.attack_speed * (1. + player_attack_speed_increases / 100.);
        self.timer
            .set_duration(Duration::from_secs_f32(1. / attack_speed));
        if self.timer.tick(delta).just_finished() {
            self.ready = true;
        }
        &self.timer
    }

    pub fn ready(&self) -> bool {
        self.ready
    }
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0}-{:.0}", self.damage_min, self.damage_max)
    }
}
