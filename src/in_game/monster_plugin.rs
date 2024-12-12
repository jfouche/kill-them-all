use crate::{components::*, schedule::*};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::ops::Mul;

#[derive(Component)]
struct FirstSpawnTimer(Timer);

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
    round: Res<Round>,
) {
    // tick the timer
    config.timer.tick(time.delta());
    if config.timer.finished() {
        let mut rng = rand::thread_rng();
        for _ in 0..config.enemy_count {
            let params = MonsterSpawnParams::generate(round.level, &mut rng);
            let params_and_assets = MonsterSpawningParamsAndAssets { params: &params };
            commands.spawn((
                MonsterFuturePos,
                Sprite::from(&params_and_assets),
                MonsterSpawnConfig::new(params),
            ));
            // commands.spawn(MonsterFuturePosBundle::new(params));
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

            let params = MonsterSpawnParamsAndAssets {
                assets: &assets,
                params: &config.params,
            };

            let monster_components = (
                MonsterRarity::from(&params),
                Sprite::from(&params),
                Transform::from(&params),
                XpOnDeath::from(&params),
            );

            match config.params.kind {
                0 => commands.spawn((MonsterType1, monster_components)),
                1 => commands.spawn((MonsterType2, monster_components)),
                2 => commands.spawn((MonsterType3, monster_components)),
                _ => unreachable!(),
            }
            .observe(send_death_event)
            .observe(increment_score);

            // commands
            //     .spawn((
            //         MonsterType1,
            //         Monster::sprite(monster_assets, &config.params),
            //         config.params.life(),
            //         config.params.movement_speed(),
            //         DamageRange::from(&config.params),
            //         config.params.xp(),
            //         Transform::from(&config.params),
            //         Monster::collider(&config.params),
            //     ))
            //     .observe(send_death_event)
            //     .observe(increment_score);
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

fn send_death_event(
    trigger: Trigger<CharacterDyingEvent>,
    mut commands: Commands,
    monsters: Query<(&Transform, &XpOnDeath), With<Monster>>,
    mut monster_death_events: EventWriter<MonsterDeathEvent>,
) {
    if let Ok((transform, xp)) = monsters.get(trigger.entity()) {
        monster_death_events.send(MonsterDeathEvent {
            pos: transform.translation,
            xp: **xp,
        });

        commands.trigger_targets(CharacterDiedEvent, trigger.entity());
    }
}

///
/// Increment score when monster died
///
fn increment_score(_trigger: Trigger<CharacterDiedEvent>, mut score: ResMut<Score>) {
    score.0 += 1;
}

///
/// Animate the monster sprite
///
fn animate_sprite(
    time: Res<Time>,
    mut q_monster: Query<(&Velocity, &mut AnimationTimer, &mut Sprite), With<Monster>>,
) {
    for (&velocity, mut timer, mut sprite) in q_monster.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
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
                };
            }
        }
    }
}
