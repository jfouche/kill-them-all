use crate::{
    in_game::collisions::{GROUP_BONUS, GROUP_ENEMY},
    prelude::*,
};
use rand::{thread_rng, Rng};
use std::ops::Mul;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets)
            .add_startup_system(init_monster_spawning)
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(monster_spawning_timer)
                    .with_system(spawn_monsters)
                    .with_system(monsters_moves)
                    .with_system(on_monster_hit)
                    .with_system(animate_sprite)
                    .with_system(increment_score),
            );
    }
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<GameTextures>,
) {
    // Monster kind 1
    let texture_handle = asset_server.load("characters/Cyclope/SpriteSheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 4, None, None);
    textures.monsters.push(texture_atlases.add(texture_atlas));

    // Monster kind 2
    let texture_handle = asset_server.load("characters/Skull/SpriteSheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 4, None, None);
    textures.monsters.push(texture_atlases.add(texture_atlas));

    // Monster kind 3
    let texture_handle = asset_server.load("characters/DragonYellow/SpriteSheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 4, None, None);
    textures.monsters.push(texture_atlases.add(texture_atlas));
}

const KIND_COUNT: usize = 3;

enum MonsterRarity {
    Normal,
    Rare,
}

/// Contains the monster informations to spawn
struct MonsterSpawnParams {
    pos: Vec2,
    rarity: MonsterRarity,
    kind: usize,
}

impl MonsterSpawnParams {
    fn rand() -> Self {
        let mut rng = thread_rng();
        // Position
        let x: f32 = rng.gen_range(-15. ..15.);
        let y: f32 = rng.gen_range(-10. ..10.);
        // Rarity
        let rarity = match rng.gen_range(0..5) {
            0 => MonsterRarity::Rare,
            _ => MonsterRarity::Normal,
        };
        // Kind
        let kind = rand::thread_rng().gen_range(0..KIND_COUNT);

        // Create the params
        MonsterSpawnParams {
            pos: Vec2::new(x, y),
            kind,
            rarity,
        }
    }

    fn size(&self) -> Vec2 {
        match self.rarity {
            MonsterRarity::Normal => Vec2::new(1.0, 1.0),
            MonsterRarity::Rare => Vec2::new(2.0, 2.0),
        }
    }

    fn life(&self) -> u16 {
        match self.rarity {
            MonsterRarity::Normal => 2,
            MonsterRarity::Rare => 5,
        }
    }
}

fn spawn_monster(
    commands: &mut Commands,
    params: &MonsterSpawnParams,
    atlas: Handle<TextureAtlas>,
) {
    let size = params.size();
    commands
        .spawn(Monster)
        .insert(Name::new("Monster"))
        .insert(MovementSpeed::new(5.0))
        .insert(Life::new(params.life()))
        // Sprite
        .insert(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(size),
                ..Default::default()
            },
            texture_atlas: atlas,
            transform: Transform::from_xyz(params.pos.x, params.pos.y, 10.),
            ..Default::default()
        })
        .insert(AnimationTimer::default())
        // Rapier
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(size.x / 2., size.y / 2.))
        .insert(CollisionGroups::new(GROUP_ENEMY, Group::ALL & !GROUP_BONUS))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::linear(Vec2::default()));
}

fn spawn_monster_futur_pos(commands: &mut Commands, params: MonsterSpawnParams) {
    commands
        .spawn(SpawningMonster)
        .insert(Name::new("Spawning monster"))
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.8, 0.3, 0.3, 0.2),
                custom_size: Some(params.size()),
                ..Default::default()
            },
            transform: Transform::from_xyz(params.pos.x, params.pos.y, 1.),
            ..Default::default()
        })
        .insert(MonsterSpawnConfig::new(params));
}

#[derive(Resource)]
struct MonsterSpawningConfig {
    timer: Timer,
    enemy_count: u16,
}
impl MonsterSpawningConfig {
    fn default() -> Self {
        MonsterSpawningConfig {
            timer: Timer::from_seconds(6., TimerMode::Repeating),
            enemy_count: 3,
        }
    }
}

#[derive(Component)]
struct MonsterSpawnConfig {
    timer: Timer,
    params: MonsterSpawnParams,
}

impl MonsterSpawnConfig {
    fn new(params: MonsterSpawnParams) -> Self {
        MonsterSpawnConfig {
            timer: Timer::from_seconds(1., TimerMode::Once),
            params,
        }
    }
}

fn init_monster_spawning(mut commands: Commands) {
    commands.insert_resource(MonsterSpawningConfig::default());
}

///
/// Spawn monster at Timer times
///
fn monster_spawning_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<MonsterSpawningConfig>,
) {
    // tick the timer
    config.timer.tick(time.delta());
    if config.timer.finished() {
        for _ in 0..config.enemy_count {
            spawn_monster_futur_pos(&mut commands, MonsterSpawnParams::rand());
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
    textures: Res<GameTextures>,
) {
    for (entity, mut config) in query.iter_mut() {
        config.timer.tick(time.delta());
        if config.timer.finished() {
            commands.entity(entity).despawn();
            spawn_monster(
                &mut commands,
                &config.params,
                textures
                    .monsters
                    .get(config.params.kind)
                    .expect("Monster type out of range !")
                    .clone(),
            );
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

///
/// Animate the monster sprite
///
fn animate_sprite(
    time: Res<Time>,
    mut q_monster: Query<(&Velocity, &mut AnimationTimer, &mut TextureAtlasSprite), With<Monster>>,
) {
    for (&velocity, mut timer, mut sprite) in q_monster.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if velocity == Velocity::zero() {
                0
            } else {
                match sprite.index {
                    0 => 4,
                    4 => 8,
                    8 => 12,
                    12 => 0,
                    _ => 0,
                }
            }
        }
    }
}
