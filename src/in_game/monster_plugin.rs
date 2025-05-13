use crate::{
    components::{
        affix::MoreLife,
        animation::AnimationTimer,
        character::{CharacterAction, CharacterDiedEvent, CharacterDyingEvent},
        damage::HitDamageRange,
        despawn_all,
        equipment::{weapon::AttackTimer, Wand},
        monster::{
            AllMonsterAssets, Monster, MonsterDeathEvent, MonsterLevel, MonsterRarity,
            MonsterSpawnParams, MonsterType1, MonsterType2, MonsterType3, ViewRange, XpOnDeath,
        },
        player::{Player, Score},
        skills::{fireball::FireBallLauncher, ActivateSkill, Skill},
        upgrade::UpgradeProvider,
        world_map::{CurrentMapLevel, SpawnMonstersEvent, LAYER_MONSTER},
    },
    schedule::{GameRunningSet, GameState},
};
use avian2d::prelude::*;
use bevy::{math::vec2, prelude::*};
use rand::Rng;
use std::f32::consts::PI;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AllMonsterAssets>()
            .init_resource::<SpawnMonsterTimer>()
            .register_type::<MonsterLevel>()
            .register_type::<ViewRange>()
            .register_type::<MonsterSpawnParams>()
            .add_event::<MonsterDeathEvent>()
            .add_systems(OnEnter(GameState::InGame), reset_monster_timer)
            .add_systems(OnExit(GameState::InGame), despawn_all::<Monster>)
            .add_systems(
                Update,
                (
                    monsters_moves,
                    animate_sprite,
                    activate_skill,
                    spawn_monster_timer,
                )
                    .in_set(GameRunningSet::EntityUpdate),
            )
            .add_observer(spawn_monsters)
            .add_observer(update_monster);
    }
}

#[derive(Resource, Deref, DerefMut)]
struct SpawnMonsterTimer(Timer);

impl Default for SpawnMonsterTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(20.0, TimerMode::Repeating))
    }
}

fn reset_monster_timer(mut timer: ResMut<SpawnMonsterTimer>) {
    timer.reset();
}

fn spawn_monster_timer(
    mut commands: Commands,
    players: Query<&Transform, With<Player>>,
    mut timer: ResMut<SpawnMonsterTimer>,
    time: Res<Time>,
    level: Res<CurrentMapLevel>,
) {
    if timer.tick(time.delta()).just_finished() {
        let Ok(player_pos) = players.single().map(|t| t.translation.xy()) else {
            error!("Single [Player] should be available");
            return;
        };

        // Spawn monsters at distance / angle of player
        let mut rng = rand::rng();
        let dist = rng.random_range(240..380) as f32;
        let angle = rng.random_range(0. ..(2. * PI));
        let pos = Vec2 {
            x: player_pos.x + dist * angle.cos(),
            y: player_pos.y + dist * angle.sin(),
        };
        let count = rng.random_range(1..=5);
        info!("spawn_monster_timer: {count} at {dist}, {angle} rad");

        // TODO: multiple group of monster, depending on map level
        let monsters = vec![(pos, count)];
        commands.trigger(SpawnMonstersEvent {
            mlevel: **level,
            monsters,
        });
    }
}

fn spawn_monsters(
    trigger: Trigger<SpawnMonstersEvent>,
    mut commands: Commands,
    assets: Res<AllMonsterAssets>,
) {
    let mut rng = rand::rng();
    let mlevel = trigger.mlevel;
    for (pos, count) in trigger.monsters.iter() {
        for i in 0..*count {
            let angle = 2. * PI * f32::from(i) / f32::from(*count);
            let dist = 20.;
            let translation = pos + dist * vec2(angle.cos(), angle.sin());
            let translation = translation.extend(LAYER_MONSTER);

            let params = MonsterSpawnParams::generate(mlevel, &mut rng);
            let scale = params.scale();

            let monster_components = (
                MonsterLevel(mlevel),
                params.rarity,
                assets.sprite(params.kind),
                Transform::from_translation(translation).with_scale(scale),
                XpOnDeath::from(&params),
                HitDamageRange::from(&params),
            );

            match params.kind {
                0 => {
                    commands.spawn((MonsterType1, monster_components));
                }
                1 => {
                    commands.spawn((MonsterType2, monster_components));
                }
                2 => {
                    commands.spawn((MonsterType3, monster_components));
                }
                _ => unreachable!(),
            }
        }
    }
}

///
/// Update monster to add affixes and observers
///
fn update_monster(
    trigger: Trigger<OnAdd, Monster>,
    mut commands: Commands,
    rarities: Query<&MonsterRarity>,
    level: Res<CurrentMapLevel>,
) {
    let monster_entity = trigger.target();
    if let Ok(MonsterRarity::Rare) = rarities.get(monster_entity) {
        let mut upgrade_provider = UpgradeProvider::new();
        let mut rng = rand::rng();
        let mut entities = Vec::new();

        for _ in 0..3 {
            if let Some(upgrade) = upgrade_provider.gen(&mut rng) {
                let upgrade_view = upgrade.generate(&mut commands, &mut rng);
                entities.push(upgrade_view.entity);
            }
        }

        commands.entity(monster_entity).add_children(&entities);

        // Add a weapon and more life
        let ilevel = **level;
        commands.spawn((Wand::new(ilevel), ChildOf(monster_entity)));
        commands.spawn((FireBallLauncher, ChildOf(monster_entity)));
        commands.spawn((MoreLife(10.), ChildOf(monster_entity)));
    }
    commands
        .entity(monster_entity)
        .observe(monster_dying)
        .observe(increment_score);
}

///
/// Monsters moves in direction of the Player
///
fn monsters_moves(
    mut monsters: Query<(&mut CharacterAction, &Transform, &ViewRange), With<Monster>>,
    players: Query<&Transform, With<Player>>,
) {
    if let Ok(player_pos) = players.single().map(|t| t.translation.xy()) {
        for (mut action, monster_transform, view_range) in &mut monsters {
            let distance = (monster_transform.translation.xy() - player_pos).length();
            // warn!("distance={distance}");
            if distance < **view_range {
                action.goto(player_pos);
            } else {
                action.stop();
            }
        }
    }
}

fn monster_dying(
    trigger: Trigger<CharacterDyingEvent>,
    monsters: Query<(&Transform, &MonsterLevel, &XpOnDeath), With<Monster>>,
    mut monster_death_events: EventWriter<MonsterDeathEvent>,
    mut character_died_events: EventWriter<CharacterDiedEvent>,
) {
    info!("monster_dying");
    if let Ok((transform, mlevel, xp)) = monsters.get(trigger.target()) {
        monster_death_events.write(MonsterDeathEvent {
            pos: transform.translation,
            xp: **xp,
            mlevel: **mlevel,
        });

        character_died_events.write(CharacterDiedEvent(trigger.target()));
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
    mut q_monster: Query<(&LinearVelocity, &mut AnimationTimer, &mut Sprite), With<Monster>>,
) {
    for (&velocity, mut timer, mut sprite) in q_monster.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if velocity == LinearVelocity::ZERO {
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

fn activate_skill(
    mut commands: Commands,
    monsters: Query<(&Transform, &ViewRange), With<Monster>>,
    players: Query<&Transform, With<Player>>,
    mut skills: Query<(Entity, &mut AttackTimer, &ChildOf), With<Skill>>,
) {
    let Ok(player_pos) = players.single().map(|t| t.translation.xy()) else {
        return;
    };
    for (entity, mut timer, child_of) in &mut skills {
        if let Ok((pos, view_range)) = monsters.get(child_of.parent()) {
            let distance = (player_pos - pos.translation.xy()).length();
            if timer.finished() && distance <= **view_range {
                commands.trigger(ActivateSkill(entity, player_pos));
                timer.reset();
            }
        }
    }
}
