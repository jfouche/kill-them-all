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
        world_map::{SpawnMonstersEvent, LAYER_MONSTER},
    },
    schedule::{GameRunningSet, GameState},
};
use bevy::{math::vec2, prelude::*};
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AllMonsterAssets>()
            .register_type::<MonsterLevel>()
            .register_type::<ViewRange>()
            .register_type::<MonsterSpawnParams>()
            .add_event::<MonsterDeathEvent>()
            .add_systems(OnExit(GameState::InGame), despawn_all::<Monster>)
            .add_systems(
                Update,
                (monsters_moves, animate_sprite, activate_skill)
                    .in_set(GameRunningSet::EntityUpdate),
            )
            .add_observer(spawn_monsters)
            .add_observer(update_monster);
    }
}

fn spawn_monsters(
    trigger: Trigger<SpawnMonstersEvent>,
    mut commands: Commands,
    assets: Res<AllMonsterAssets>,
) {
    let mut rng = rand::rng();
    let mlevel = trigger.event().mlevel;
    for (pos, count) in trigger.event().monsters.iter() {
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
) {
    if let Ok(MonsterRarity::Rare) = rarities.get(trigger.entity()) {
        let mut upgrade_provider = UpgradeProvider::new();
        let mut rng = rand::rng();
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
            c.spawn(Wand::new(1)); // TODO: get ilevel
            c.spawn(FireBallLauncher);
            c.spawn(MoreLife(10.));
        });
    }
    commands
        .entity(trigger.entity())
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
    if let Ok(player_pos) = players.get_single().map(|t| t.translation.xy()) {
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
    if let Ok((transform, mlevel, xp)) = monsters.get(trigger.entity()) {
        monster_death_events.send(MonsterDeathEvent {
            pos: transform.translation,
            xp: **xp,
            mlevel: **mlevel,
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

fn activate_skill(
    mut commands: Commands,
    monsters: Query<(&Transform, &ViewRange), With<Monster>>,
    players: Query<&Transform, With<Player>>,
    mut skills: Query<(Entity, &mut AttackTimer, &Parent), With<Skill>>,
) {
    let Ok(player_pos) = players.get_single().map(|t| t.translation.xy()) else {
        return;
    };
    for (entity, mut timer, parent) in &mut skills {
        if let Ok((pos, view_range)) = monsters.get(parent.get()) {
            let distance = (player_pos - pos.translation.xy()).length();
            if timer.finished() && distance <= **view_range {
                commands.trigger(ActivateSkill(entity, player_pos));
                timer.reset();
            }
        }
    }
}
