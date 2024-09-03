mod bonus;
mod monster;
mod player;
mod skills;
mod upgrade;
mod weapon;
mod world_map;

pub use bonus::*;
pub use monster::*;
pub use player::*;
pub use skills::*;
pub use upgrade::*;
pub use weapon::*;
pub use world_map::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub const GROUP_PLAYER: Group = Group::GROUP_1;
pub const GROUP_ENEMY: Group = Group::GROUP_2;
pub const GROUP_BONUS: Group = Group::GROUP_3;
pub const GROUP_BULLET: Group = Group::GROUP_4;

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_all<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

// ==================================================================
// Money

#[derive(Component, Deref, DerefMut, Reflect)]
pub struct Money(pub u16);

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ==================================================================
// Experience

#[derive(Component, Default, Reflect)]
pub struct Experience(u32);

impl Experience {
    const LEVELS: [u32; 6] = [2, 10, 40, 100, 400, 1000];

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
// LifeTime

#[derive(Component, Deref, DerefMut)]
pub struct LifeTime(Timer);

impl LifeTime {
    pub fn new(secs: f32) -> Self {
        LifeTime(Timer::from_seconds(secs, TimerMode::Once))
    }
}

// ==================================================================
// ScoreResource

#[derive(Default, Resource)]
pub struct ScoreResource(pub u16);

// ==================================================================
// Round

#[derive(Resource)]
pub struct Round {
    pub level: u16,
    pub timer: Timer,
}

impl Default for Round {
    fn default() -> Self {
        Round {
            level: 0,
            timer: Timer::from_seconds(60., TimerMode::Repeating),
        }
    }
}
