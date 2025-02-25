pub mod affix;
pub mod animation;
pub mod character;
pub mod damage;
pub mod equipment;
pub mod inventory;
pub mod item;
pub mod monster;
pub mod orb;
pub mod player;
pub mod rng_provider;
pub mod skills;
pub mod upgrade;
pub mod world_map;

pub use common::{
    despawn_all, LifeTime, GROUP_ALL, GROUP_DAMAGER, GROUP_ENEMY, GROUP_ITEM, GROUP_PLAYER,
};

mod common {
    use bevy::prelude::*;
    use bevy_rapier2d::prelude::Group;

    pub const GROUP_PLAYER: Group = Group::GROUP_1;
    pub const GROUP_ENEMY: Group = Group::GROUP_2;
    pub const GROUP_ITEM: Group = Group::GROUP_3;
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
}
