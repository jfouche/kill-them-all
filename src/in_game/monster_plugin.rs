use crate::{components::*, schedule::*};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawningMonsterAssets>()
            .init_resource::<AllMonsterAssets>()
            .register_type::<ViewRange>()
            .register_type::<MonsterSpawnParams>()
            .add_event::<MonsterDeathEvent>()
            .add_systems(OnExit(GameState::InGame), despawn_all::<Monster>)
            .add_systems(
                Update,
                (spawn_monsters, monsters_moves, animate_sprite)
                    .in_set(GameRunningSet::EntityUpdate),
            )
            .add_observer(add_affixes);
    }
}

///
/// Spawn monster
///
fn spawn_monsters(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &Transform,
        &mut MonsterSpawnTimer,
        &MonsterSpawnParams,
    )>,
    assets: Res<AllMonsterAssets>,
) {
    for (entity, transform, mut timer, params) in query.iter_mut() {
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
                Transform::from_translation(transform.translation),
                XpOnDeath::from(&params_assets),
                HitDamageRange::from(&params_assets),
            );

            match params.kind {
                0 => commands.spawn((MonsterType1, monster_components)),
                1 => commands.spawn((MonsterType2, monster_components)),
                2 => commands.spawn((MonsterType3, monster_components)),
                _ => unreachable!(),
            }
            .observe(monster_dying)
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
    mut monsters: Query<(&mut NextPosition, &Transform, &ViewRange), With<Monster>>,
    players: Query<&Transform, With<Player>>,
) {
    if let Ok(player_pos) = players.get_single().map(|t| t.translation.xy()) {
        for (mut next_pos, monster_transform, view_range) in &mut monsters {
            let distance = (monster_transform.translation.xy() - player_pos).length();
            // warn!("distance={distance}");
            if distance < **view_range {
                next_pos.goto(player_pos);
            } else {
                next_pos.stop();
            }
        }
    }
}

fn monster_dying(
    trigger: Trigger<CharacterDyingEvent>,
    monsters: Query<(&Transform, &XpOnDeath), With<Monster>>,
    mut monster_death_events: EventWriter<MonsterDeathEvent>,
    mut character_died_events: EventWriter<CharacterDiedEvent>,
) {
    info!("monster_dying");
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
