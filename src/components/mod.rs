mod affix;
mod animation;
mod bonus;
mod character;
mod equipment;
mod monster;
mod player;
mod rng_provider;
mod skills;
mod upgrade;
mod world_map;

pub use affix::*;
pub use animation::*;
use bevy_rapier2d::prelude::Group;
pub use bonus::*;
pub use character::*;
pub use equipment::*;
pub use monster::*;
pub use player::*;
pub use skills::*;
pub use upgrade::*;
pub use world_map::*;

use bevy::prelude::*;

pub const GROUP_PLAYER: Group = Group::GROUP_1;
pub const GROUP_ENEMY: Group = Group::GROUP_2;
pub const GROUP_BONUS: Group = Group::GROUP_3;
pub const GROUP_DAMAGER: Group = Group::GROUP_4;
pub const GROUP_ALL: Group = Group::ALL;

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
