use crate::{components::*, schedule::*};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::ops::Mul;

#[derive(Resource)]
struct MonsterSpawnerTimer {
    timer: Timer,
    enemy_count: u16,
}

impl Default for MonsterSpawnerTimer {
    fn default() -> Self {
        MonsterSpawnerTimer {
            timer: Timer::from_seconds(6., TimerMode::Repeating),
            enemy_count: 3,
        }
    }
}

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawningMonsterAssets>()
            .init_resource::<AllMonsterAssets>()
            .init_resource::<MonsterSpawnerTimer>()
            .register_type::<MonsterSpawnParams>()
            .add_event::<MonsterDeathEvent>()
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
            )
            .add_observer(add_affixes);
    }
}

///
/// Spawn monster at Timer times
///
fn monster_spawning_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<MonsterSpawnerTimer>,
    spawning_assets: Res<SpawningMonsterAssets>,
    round: Res<Round>,
) {
    // tick the timer
    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.finished() {
        let mut rng = rand::thread_rng();
        for _ in 0..spawn_timer.enemy_count {
            let params = MonsterSpawnParams::generate(round.level, &mut rng);
            commands.spawn((
                MonsterFuturePos,
                Transform::from(&params),
                Mesh2d::from(&*spawning_assets),
                MeshMaterial2d::from(&*spawning_assets),
                params,
            ));
        }
        spawn_timer.enemy_count += 1;
    }
}

///
/// Spawn monster at Timer times
///
fn spawn_monsters(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut MonsterSpawnTimer, &MonsterSpawnParams)>,
    assets: Res<AllMonsterAssets>,
) {
    for (entity, mut timer, params) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            commands.entity(entity).despawn();

            let params_assets = MonsterSpawnParamsAndAssets {
                assets: &assets,
                params,
            };

            let monster_components = (
                MonsterRarity::from(&params_assets),
                Sprite::from(&params_assets),
                Transform::from(params_assets.params),
                XpOnDeath::from(&params_assets),
                HitDamageRange::from(&params_assets),
            );

            match params.kind {
                0 => commands.spawn((MonsterType1, monster_components)),
                1 => commands.spawn((MonsterType2, monster_components)),
                2 => commands.spawn((MonsterType3, monster_components)),
                _ => unreachable!(),
            }
            .observe(send_death_event)
            .observe(increment_score);
        }
    }
}

///
/// Add affixes to rare monsters
///
fn add_affixes(
    trigger: Trigger<OnAdd, Monster>,
    mut commands: Commands,
    rarities: Query<&MonsterRarity>,
) {
    if let Ok(MonsterRarity::Rare) = rarities.get(trigger.entity()) {
        info!("add_affixes()");
        let mut upgrade_provider = UpgradeProvider::new();
        let mut rng = rand::thread_rng();
        let mut entities = Vec::new();

        for _ in 0..3 {
            if let Some(upgrade) = upgrade_provider.gen(&mut rng) {
                let upgrade_view = upgrade.generate(&mut commands, &mut rng);
                entities.push(upgrade_view.entity);
            }
        }

        commands.entity(trigger.entity()).add_children(&entities);

        // Add a weapon and more life
        commands.entity(trigger.entity()).with_children(|c| {
            c.spawn(Wand);
            c.spawn(MoreLife(10.));
        });
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
    monsters: Query<(&Transform, &XpOnDeath), With<Monster>>,
    mut monster_death_events: EventWriter<MonsterDeathEvent>,
    mut character_died_events: EventWriter<CharacterDiedEvent>,
) {
    if let Ok((transform, xp)) = monsters.get(trigger.entity()) {
        monster_death_events.send(MonsterDeathEvent {
            pos: transform.translation,
            xp: **xp,
        });

        character_died_events.send(CharacterDiedEvent(trigger.entity()));
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
