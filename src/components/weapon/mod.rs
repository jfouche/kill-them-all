mod gun;
pub use gun::*;

mod shuriken;
pub use shuriken::*;

use super::*;
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

#[derive(Component, Default, Deref)]
pub struct Damage(pub u16);
pub enum WeaponType {
    Gun,
    _Shuriken,
}

#[derive(Component)]
pub struct Weapon {
    _weapon_type: WeaponType,
    /// Attack per second
    attack_speed: f32,
    damage_min: u16,
    damage_max: u16,
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
        damage_min: u16,
        damage_max: u16,
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

    pub fn attack(&mut self) -> u16 {
        self.ready = false;
        rand::thread_rng().gen_range(self.damage_min..=self.damage_max)
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
        write!(f, "{}-{}", self.damage_min, self.damage_max)
    }
}
