mod bonus;
mod equipment;
mod monster;
mod player;
mod rng_provider;
mod skill;
mod upgrade;
mod weapon;
mod world_map;

pub use bonus::*;
pub use equipment::*;
pub use monster::*;
pub use player::*;
pub use skill::*;
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

#[derive(Default, Resource, Reflect)]
pub struct Score(pub u16);

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
            timer: Timer::from_seconds(15., TimerMode::Repeating),
        }
    }
}
