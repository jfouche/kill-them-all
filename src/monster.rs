use crate::components::*;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_monster_spawning)
            .add_system(spawn_monsters);
    }
}

#[derive(Bundle)]
struct MonsterBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    monster: Monster,
}

impl MonsterBundle {
    fn from_xy(x: f32, y: f32) -> Self {
        MonsterBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.3, 0.3),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, y, 1.),
                ..Default::default()
            },
            monster: Monster,
        }
    }
}

struct MonsterSpawnConfig {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
}

fn init_monster_spawning(mut commands: Commands) {
    commands.insert_resource(MonsterSpawnConfig {
        // create the repeating timer
        timer: Timer::from_seconds(5., true),
    });
}

fn spawn_monsters(mut commands: Commands, time: Res<Time>, mut config: ResMut<MonsterSpawnConfig>) {
    // tick the timer
    config.timer.tick(time.delta());

    if config.timer.finished() {
        let mut rng = thread_rng();
        for _ in 0..5 {
            let x: f32 = rng.gen_range(-5. .. 5.);
            let y: f32 = rng.gen_range(-3. .. 3.);
            commands
                .spawn_bundle(MonsterBundle::from_xy(x, y))
                .insert(Name::new("Enemy"));
        }
    }
}
