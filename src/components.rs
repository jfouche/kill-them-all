use crate::prelude::*;
use rand::Rng;
use std::{cmp::min, time::Duration};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct SpawningMonster;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Damage(pub u16);

// ==================================================================
// AnimationTimer

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

// ==================================================================
// Weapon

#[derive(Component)]
pub struct Weapon {
    attack_speed: f32,
    damage_min: u16,
    damage_max: u16,
    timer: Timer,
    ready: bool,
}

impl Weapon {
    pub fn new(attack_per_second: f32, damage_min: u16, damage_max: u16) -> Self {
        Weapon {
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

// ==================================================================
// MovementSpeed

#[derive(Component)]
pub struct MovementSpeed {
    speed: f32,
    increases: f32,
}

impl MovementSpeed {
    pub fn new(speed: f32) -> Self {
        MovementSpeed {
            speed,
            increases: 0.0,
        }
    }
    pub fn value(&self) -> f32 {
        self.speed * (100.0 + self.increases) / 100.0
    }

    pub fn increases(&mut self, percent: f32) {
        self.increases += percent;
    }
}

impl std::fmt::Display for MovementSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}    {:.0}%", self.value(), self.increases)
    }
}

// ==================================================================
// Life

#[derive(Component)]
pub struct Life {
    life: u16,
    max_life: u16,
    increases: f32,
}

impl Life {
    pub fn new(life: u16) -> Self {
        Life {
            life,
            max_life: life,
            increases: 0.,
        }
    }

    pub fn hit(&mut self, damage: u16) {
        if damage > self.life {
            self.life = 0;
        } else {
            self.life -= damage;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.life == 0
    }

    pub fn life(&self) -> u16 {
        self.life
    }

    pub fn max_life(&self) -> u16 {
        (self.max_life as f32 * (100.0 + self.increases) / 100.0) as u16
    }

    pub fn increases(&mut self, percent: f32) {
        self.increases += percent;
    }

    pub fn regenerate(&mut self, life: u16) {
        self.life = min(self.max_life(), self.life + life);
    }
}

impl std::fmt::Display for Life {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}    +{}%",
            self.life(),
            self.max_life(),
            self.increases
        )
    }
}

// ==================================================================
// AttackSpeed

#[derive(Component)]
pub struct AttackSpeed {
    increases: f32,
}

impl AttackSpeed {
    pub fn new() -> Self {
        AttackSpeed { increases: 0.0 }
    }

    pub fn value(&self) -> f32 {
        self.increases
    }

    pub fn increases(&mut self, percent: f32) {
        self.increases += percent;
    }
}

impl std::fmt::Display for AttackSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " +{:.0}%", self.increases)
    }
}

// ==================================================================
// Bonus

#[derive(Component)]
pub struct Bonus;

// ==================================================================
// Money

#[derive(Component, Reflect)]
pub struct Money(pub u16);

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ==================================================================
// Experience

#[derive(Component, Default)]
pub struct Experience(u32);

impl Experience {
    const LEVELS: [u32; 6] = [4, 10, 40, 100, 400, 1000];

    pub fn add(&mut self, xp: u32) {
        self.0 += xp;
    }

    pub fn current(&self) -> u32 {
        self.0
    }

    /// Level starting at 0
    pub fn level(&self) -> u8 {
        let mut level = 0;
        for xp in Experience::LEVELS.iter() {
            if self.0 >= *xp {
                level += 1;
            } else {
                break;
            }
        }
        level
    }

    pub fn get_current_level_min_max_exp(&self) -> (u32, u32) {
        let level = self.level();
        let min = match level {
            0 => &0,
            _ => Experience::LEVELS.get(level as usize - 1).unwrap_or(&100),
        };
        let max = Experience::LEVELS
            .get(level as usize)
            .unwrap_or(Experience::LEVELS.last().unwrap());
        (*min, *max)
    }
}

impl std::fmt::Display for Experience {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{} (level {})",
            self.0,
            self.get_current_level_min_max_exp().1,
            self.level() + 1,
        )
    }
}
