use crate::prelude::*;
use rand::{thread_rng, Rng};
use std::ops::Mul;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_monster_spawning)
            .add_system(spawning_monsters)
            .add_system(spawn_monsters)
            .add_system(monsters_moves)
            .add_system(on_monster_hit)
            .add_system(on_monster_death);
    }
}

#[derive(Bundle)]
struct MonsterBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    monster: Monster,
    speed: Speed,
    life: Life,
    body: RigidBody,
    collider: Collider,
    gravity: GravityScale,
    constraints: LockedAxes,
    velocity: Velocity,
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
            monster: Monster,
            speed: Speed(5.0),
            life: Life::new(2),
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(size.x / 2., size.y / 2.),
            gravity: GravityScale(0.0),
            constraints: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::linear(Vec2::default()),
        }
    }
}

#[derive(Bundle)]
struct SpawningMonsterBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    monster: SpawningMonster,
    config: MonsterSpawnConfig,
}

impl SpawningMonsterBundle {
    fn from_xy(x: f32, y: f32) -> Self {
        let size = Vec2::new(1., 1.);
        SpawningMonsterBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.8, 0.3, 0.3, 0.2),
                    custom_size: Some(size),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, y, 1.),
                ..Default::default()
            },
            monster: SpawningMonster,
            config: MonsterSpawnConfig::new(x, y),
        }
    }
}

#[derive(Resource)]
struct MonsterSpawningConfig {
    timer: Timer,
    enemy_count: u16,
}
impl MonsterSpawningConfig {
    fn default() -> Self {
        MonsterSpawningConfig {
            timer: Timer::from_seconds(8., TimerMode::Repeating),
            enemy_count: 5,
        }
    }
}

#[derive(Component)]
struct MonsterSpawnConfig {
    timer: Timer,
    x: f32,
    y: f32,
}

impl MonsterSpawnConfig {
    fn new(x: f32, y: f32) -> Self {
        MonsterSpawnConfig {
            timer: Timer::from_seconds(1., TimerMode::Once),
            x,
            y,
        }
    }
}

fn init_monster_spawning(mut commands: Commands) {
    commands.insert_resource(MonsterSpawningConfig::default());
}

///
/// Spawn monster at Timer times
///
fn spawning_monsters(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<MonsterSpawningConfig>,
) {
    // tick the timer
    config.timer.tick(time.delta());

    if config.timer.finished() {
        let mut rng = thread_rng();
        for _ in 0..config.enemy_count {
            let x: f32 = rng.gen_range(-15. ..15.);
            let y: f32 = rng.gen_range(-10. ..10.);
            commands
                .spawn(SpawningMonsterBundle::from_xy(x, y))
                .insert(Name::new("Enemy spawning"));
        }
        config.enemy_count += 2;
    }
}

///
/// Spawn monster at Timer times
///
fn spawn_monsters(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut MonsterSpawnConfig)>,
) {
    for (entity, mut config) in query.iter_mut() {
        config.timer.tick(time.delta());
        if config.timer.finished() {
            commands.entity(entity).despawn();

            commands
                .spawn(MonsterBundle::from_xy(config.x, config.y))
                .insert(Name::new("Enemy"));
        }
    }
}

///
/// Monsters moves in direction of the Player
///
fn monsters_moves(
    mut q_monsters: Query<(&Transform, &mut Velocity, &Speed), (With<Monster>, Without<Player>)>,
    q_player: Query<&Transform, With<Player>>,
) {
    if let Ok(player) = q_player.get_single() {
        for (transform, mut velocity, speed) in q_monsters.iter_mut() {
            let direction = player.translation - transform.translation;
            let offset = Vec2::new(direction.x, direction.y);
            velocity.linvel = offset.normalize_or_zero().mul(**speed);
        }
    }
}

///
/// monster hit
///
fn on_monster_hit(
    mut monster_hit_events: EventReader<MonsterHitEvent>,
    mut q_monsters: Query<(Entity, &mut Life, &Transform), With<Monster>>,
    mut monster_death_events: EventWriter<MonsterDeathEvent>,
) {
    for event in monster_hit_events.iter() {
        warn!("on_monster_hit");
        for (entity, mut life, transform) in q_monsters.iter_mut() {
            if entity == event.entity {
                life.hit(1);
                if life.is_dead() {
                    monster_death_events.send(MonsterDeathEvent {
                        entity,
                        pos: transform.translation,
                    })
                }
            }
        }
    }
}

///
/// monster died
///
fn on_monster_death(
    mut commands: Commands,
    mut monster_hit_events: EventReader<MonsterDeathEvent>,
    mut score: ResMut<ScoreResource>,
) {
    for event in monster_hit_events.iter() {
        warn!("on_monster_death");
        commands.entity(event.entity).despawn();
        score.0 += 1;
    }
}
