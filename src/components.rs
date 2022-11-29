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

#[derive(Component, Default)]
pub struct Experience(u32);

impl Experience {
    const LEVELS: [u32; 4] = [4, 10, 40, 100];
    const MAX_LEVEL: u8 = Experience::LEVELS.len() as u8;

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
        let max = Experience::LEVELS.get(level as usize).unwrap_or(&100);
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
            self.level(),
        )
    }
}
