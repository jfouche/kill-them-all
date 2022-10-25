use std::ops::Mul;

use crate::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_monster_spawning)
            .add_system(spawn_monsters)
            .add_system(monsters_moves)
            ;
    }
}

#[derive(Bundle)]
struct MonsterBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    monster: Monster,
    body: RigidBody,
    collider: Collider,
    gravity: GravityScale,
    constraints: LockedAxes,
    velocity: Velocity
}

impl MonsterBundle {
    fn from_xy(x: f32, y: f32) -> Self {
        let size = Vec2::new(1., 1.);
        MonsterBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.3, 0.3),
                    custom_size: Some(size),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, y, 1.),
                ..Default::default()
            },
            monster: Monster { speed: 6. },
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(size.x / 2., size.y / 2.),
            gravity: GravityScale(0.0),
            constraints: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::linear(Vec2::default())
        }
    }
}

struct MonsterSpawnConfig {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
}

fn init_monster_spawning(mut commands: Commands) {
    commands.insert_resource(MonsterSpawnConfig {
        timer: Timer::from_seconds(5., true),
    });
}
///
/// Spawn monster at Timer times
/// 
fn spawn_monsters(mut commands: Commands, time: Res<Time>, mut config: ResMut<MonsterSpawnConfig>) {
    // tick the timer
    config.timer.tick(time.delta());

    if config.timer.finished() {
        let mut rng = thread_rng();
        for _ in 0..5 {
            let x: f32 = rng.gen_range(-15. .. 15.);
            let y: f32 = rng.gen_range(-10. .. 10.);
            commands
                .spawn_bundle(MonsterBundle::from_xy(x, y))
                .insert(Name::new("Enemy"));
        }
    }
}

///
/// Monsters moves in direction of the Player
/// 
fn monsters_moves(mut q_monsters: Query<(&Transform, &mut Velocity, &Monster), Without<Player>>, q_player: Query<&Transform, With<Player>>) {
    let player = q_player.single();

    for (transform, mut velocity, monster) in q_monsters.iter_mut() {
        let direction = player.translation - transform.translation;
        let offset = direction.normalize().mul(monster.speed);
        velocity.linvel = Vec2::new(offset.x, offset.y);
    }
}
