use crate::{
    in_game::collisions::{GROUP_BONUS, GROUP_ENEMY},
    prelude::*,
};
use rand::{thread_rng, Rng};
use std::ops::Mul;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_monster_spawning)
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(spawning_monsters)
                    .with_system(spawn_monsters)
                    .with_system(monsters_moves)
                    .with_system(on_monster_hit)
                    .with_system(increment_score),
            );
    }
}

const MONSTER_SIZE: Vec2 = Vec2::new(1.0, 1.0);

fn spawn_monster(commands: &mut Commands, x: f32, y: f32) {
    commands
        .spawn(Monster)
        .insert(Name::new("Monster"))
        .insert(MovementSpeed::new(5.0))
        .insert(Life::new(2))
        // Sprite
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.3, 0.3),
                custom_size: Some(MONSTER_SIZE),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 1.),
            ..Default::default()
        })
        // Rapier
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(MONSTER_SIZE.x / 2., MONSTER_SIZE.y / 2.))
        .insert(CollisionGroups::new(GROUP_ENEMY, Group::ALL & !GROUP_BONUS))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::linear(Vec2::default()));
}

fn spawning_monster(commands: &mut Commands, x: f32, y: f32) {
    commands
        .spawn(SpawningMonster)
        .insert(Name::new("Spawning monster"))
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.8, 0.3, 0.3, 0.2),
                custom_size: Some(MONSTER_SIZE),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 1.),
            ..Default::default()
        })
        .insert(MonsterSpawnConfig::new(x, y));
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
            enemy_count: 3,
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
            spawning_monster(&mut commands, x, y);
        }
        config.enemy_count += 1;
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
            spawn_monster(&mut commands, config.x, config.y);
        }
    }
}

///
/// Monsters moves in direction of the Player
///
fn monsters_moves(
    mut q_monsters: Query<
        (&Transform, &mut Velocity, &MovementSpeed),
        (With<Monster>, Without<Player>),
    >,
    q_player: Query<&Transform, With<Player>>,
) {
    if let Ok(player) = q_player.get_single() {
        for (transform, mut velocity, speed) in q_monsters.iter_mut() {
            let direction = player.translation - transform.translation;
            let offset = Vec2::new(direction.x, direction.y);
            velocity.linvel = offset.normalize_or_zero().mul(speed.value());
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
                life.hit(event.damage);
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
/// Increment score when monster died
///
fn increment_score(
    mut commands: Commands,
    mut monster_hit_events: EventReader<MonsterDeathEvent>,
    mut score: ResMut<ScoreResource>,
) {
    for event in monster_hit_events.iter() {
        warn!("increment_score");
        // TODO: ("split in 2 systems");
        commands.entity(event.entity).despawn();
        score.0 += 1;
    }
}
