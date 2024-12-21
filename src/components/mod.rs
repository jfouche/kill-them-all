mod animation;
mod bonus;
mod character;
mod equipment;
mod monster;
mod player;
mod rng_provider;
mod upgrade;
mod weapon;
mod world_map;

pub use animation::*;
pub use bonus::*;
pub use character::*;
pub use equipment::*;
pub use monster::*;
pub use player::*;
pub use upgrade::*;
pub use weapon::*;
pub use world_map::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub const GROUP_PLAYER: Group = Group::GROUP_1;
pub const GROUP_ENEMY: Group = Group::GROUP_2;
pub const GROUP_BONUS: Group = Group::GROUP_3;
pub const GROUP_DAMAGER: Group = Group::GROUP_4;

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_all<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

///
/// The [LifeTime] component indicates that the entity should be depawn
/// after a certain duration
///

#[derive(Component, Deref, DerefMut)]
pub struct LifeTime(Timer);

impl LifeTime {
    pub fn new(secs: f32) -> Self {
        LifeTime(Timer::from_seconds(secs, TimerMode::Once))
    }
}

///
/// A [Round] is fixed time period when monster spawn
///
#[derive(Resource, Reflect)]
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
