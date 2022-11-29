use crate::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Monster;

#[derive(Component, Deref)]
pub struct Speed(pub f32);

impl std::fmt::Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
}

impl Life {
    pub fn new(life: u16) -> Self {
        Life {
            life,
            max_life: life,
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

    // pub fn max_life(&self) -> u16 {
    //     self.max_life
    // }

    pub fn percent(&self) -> f32 {
        100.0 * self.life as f32 / self.max_life as f32
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
