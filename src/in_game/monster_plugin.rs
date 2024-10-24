use crate::{components::*, schedule::*};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::ops::Mul;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MonsterDeathEvent>()
            .add_systems(Startup, (load_assets, init_monster_spawning))
            .add_systems(OnExit(GameState::InGame), despawn_all::<Monster>)
            .add_systems(
                Update,
                (
                    monster_spawning_timer,
                    spawn_monsters,
                    monsters_moves,
                    animate_sprite,
                    increment_score,
                )
                    .in_set(GameRunningSet::EntityUpdate),
            );
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut all_monster_assets = AllMonsterAssets::default();

    let texture_atlas_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        4,
        4,
        None,
        None,
    ));

    // Monster kind 1
    let texture = asset_server.load("characters/Cyclope/SpriteSheet.png");
    all_monster_assets.push(MonsterAssets {
        texture,
        texture_atlas_layout: texture_atlas_layout.clone(),
    });

    // Monster kind 2
    let texture = asset_server.load("characters/Skull/SpriteSheet.png");
    all_monster_assets.push(MonsterAssets {
        texture,
        texture_atlas_layout: texture_atlas_layout.clone(),
    });

    // Monster kind 3
    let texture = asset_server.load("characters/DragonYellow/SpriteSheet.png");
    all_monster_assets.push(MonsterAssets {
        texture,
        texture_atlas_layout,
    });

    commands.insert_resource(all_monster_assets);
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
            commands.spawn(MonsterFuturePosBundle::new(MonsterSpawnParams::rand()));
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
    assets: Res<AllMonsterAssets>,
) {
    for (entity, mut config) in query.iter_mut() {
        config.timer.tick(time.delta());
        if config.timer.finished() {
            commands.entity(entity).despawn();
            let monster_assets = assets
                .get(config.params.kind)
                .expect("Monster type out of range !");

            commands
                .spawn(MonsterBundle::new(monster_assets, &config.params))
                .observe(trigger_monster_hit);
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
            velocity.linvel = offset.normalize_or_zero().mul(**speed);
        }
    }
}

///
/// monster hit
///
fn trigger_monster_hit(
    hit_event: Trigger<HitEvent>,
    mut q_monsters: Query<(&mut Life, &Transform, &XpOnDeath), With<Monster>>,
    mut monster_death_events: EventWriter<MonsterDeathEvent>,
) {
    info!("on_monster_hit");
    let entity = hit_event.entity();
    if let Ok((mut life, transform, xp)) = q_monsters.get_mut(entity) {
        life.hit(hit_event.event().damage);
        if life.is_dead() {
            monster_death_events.send(MonsterDeathEvent {
                entity,
                pos: transform.translation,
                xp: **xp,
            });
        }
    }
}

///
/// Increment score when monster died
///
fn increment_score(
    mut commands: Commands,
    mut monster_hit_events: EventReader<MonsterDeathEvent>,
    mut score: ResMut<Score>,
) {
    for event in monster_hit_events.read() {
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
    mut q_monster: Query<(&Velocity, &mut AnimationTimer, &mut TextureAtlas), With<Monster>>,
) {
    for (&velocity, mut timer, mut atlas) in q_monster.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if velocity == Velocity::zero() {
                0
            } else {
                match atlas.index {
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
