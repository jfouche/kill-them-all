use std::cmp::min;

use crate::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Monster;

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
        write!(f, "{}\t+{:.0}", self.speed, self.increases)
    }
}

#[derive(Component)]
pub struct SpawningMonster;

#[derive(Component)]
pub struct Bullet;

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
        write!(f, "{}/{} +{}%", self.life, self.max_life(), self.increases)
    }
}

#[derive(Component)]
pub struct Invulnerable {
    pub filters: Group,
    timer: Timer,
}

impl Invulnerable {
    pub fn new(duration: f32, filters: Group) -> Self {
        Invulnerable {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            filters,
        }
    }

    pub fn tick_and_finished(&mut self, time: Res<Time>) -> bool {
        self.timer.tick(time.delta());
        self.timer.finished()
    }
}

#[derive(Component)]
pub struct Bonus;

#[derive(Component, Reflect)]
pub struct Money(pub u16);

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
