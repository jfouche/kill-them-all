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

pub use common::{despawn_all, GameLayer, LifeTime};

mod common {
    use avian2d::prelude::*;
    use bevy::prelude::*;

    #[derive(PhysicsLayer, Default)]
    pub enum GameLayer {
        #[default]
        Ground,
        Player,
        Enemy,
        Item,
        Damager,
    }

    /// Generic system that takes a component as a parameter, and will despawn all entities with that component
    pub fn despawn_all<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
        for entity in &to_despawn {
            commands.entity(entity).despawn();
        }
    }

    pub trait EntityInserter {
        fn insert<B: Bundle>(&mut self, bundle: B);
    }

    impl EntityInserter for EntityWorldMut<'_> {
        fn insert<B: Bundle>(&mut self, bundle: B) {
            EntityWorldMut::insert(self, bundle);
        }
    }

    impl EntityInserter for EntityCommands<'_> {
        fn insert<B: Bundle>(&mut self, bundle: B) {
            EntityCommands::insert(self, bundle);
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
