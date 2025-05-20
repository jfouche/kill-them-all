use crate::{
    components::{
        affix::MoreLife,
        animation::AnimationTimer,
        character::{CharacterAction, CharacterDiedEvent, CharacterDyingEvent},
        despawn_all,
        equipment::{weapon::AttackTimer, Wand},
        item::ItemSpawner,
        monster::{
            AllMonsterAssets, Monster, MonsterBuilder, MonsterDeathEvent, MonsterLevel,
            MonsterRarity, MonsterType1, MonsterType2, MonsterType3, SpawnMonstersEvent, ViewRange,
            XpOnDeath,
        },
        player::{Player, Score},
        skills::{
            death_aura::DeathAura, fireball::FireBallLauncher, shuriken::ShurikenLauncher,
            ActivateSkill, Skill,
        },
        upgrade::UpgradeProvider,
        world_map::CurrentMapLevel,
    },
    schedule::{GameRunningSet, GameState},
};
use bevy::{math::vec2, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AllMonsterAssets>()
            .init_resource::<SpawnMonsterTimer>()
            .register_type::<MonsterLevel>()
            .register_type::<ViewRange>()
            .register_type::<MonsterBuilder>()
            .add_event::<MonsterDeathEvent>()
            .add_event::<SpawnMonstersEvent>()
            .add_systems(OnEnter(GameState::InGame), reset_monster_timer)
            .add_systems(OnExit(GameState::InGame), despawn_all::<Monster>)
            .add_systems(
                Update,
                (
                    spawn_monsters,
                    monsters_moves,
                    animate_sprite,
                    activate_skill,
                    spawn_monster_timer,
                )
                    .in_set(GameRunningSet::EntityUpdate),
            )
            .add_observer(update_monster)
            .add_observer(customize_monster_type_1)
            .add_observer(customize_monster_type_2)
            .add_observer(customize_monster_type_3);
    }
}

#[derive(Resource, Deref, DerefMut)]
struct SpawnMonsterTimer(Timer);

impl Default for SpawnMonsterTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(12.0, TimerMode::Repeating))
    }
}

fn reset_monster_timer(mut timer: ResMut<SpawnMonsterTimer>) {
    timer.reset();
}

fn spawn_monster_timer(
    players: Query<&Transform, With<Player>>,
    mut timer: ResMut<SpawnMonsterTimer>,
    time: Res<Time>,
    mlevel: Res<CurrentMapLevel>,
    mut spawn_monsters: EventWriter<SpawnMonstersEvent>,
) -> Result {
    if !timer.tick(time.delta()).just_finished() {
        return Ok(());
    }

    let player_pos = players.single().map(|t| t.translation.xy())?;

    let mut rng = rand::rng();
    let monsters_to_spawn = mlevel.monsters_to_spawn(&mut rng);

    for _ in 0..monsters_to_spawn.n_groups {
        // Spawn monsters at random distance / angle of player
        let dist = rng.random_range(220..320) as f32;
        let angle = rng.random_range(0..100) as f32 * 2.0 * PI / 100.;
        let pos = Vec2 {
            x: player_pos.x + dist * angle.cos(),
            y: player_pos.y + dist * angle.sin(),
        };
        let count = monsters_to_spawn.n_monsters;
        info!("spawn_monster_timer: {count} monsters");

        spawn_monsters.write(SpawnMonstersEvent {
            mlevel: **mlevel,
            monsters: vec![(pos, count)],
        });
    }
    Ok(())
}

fn spawn_monsters(
    mut commands: Commands,
    mut monsters_to_spawn_reader: EventReader<SpawnMonstersEvent>,
    assets: Res<AllMonsterAssets>,
) {
    let mut rng = rand::rng();
    for monsters_to_spawn in monsters_to_spawn_reader.read() {
        for (pos, count) in monsters_to_spawn.monsters.iter() {
            for i in 0..*count {
                let angle = 2. * PI * f32::from(i) / f32::from(*count);
                let dist = 20.;
                let pos = pos + dist * vec2(angle.cos(), angle.sin());

                let monster_builder = MonsterBuilder::generate(monsters_to_spawn.mlevel, &mut rng);
                match monster_builder.kind {
                    0 => {
                        commands.spawn(MonsterType1::bundle(monster_builder, pos, &assets));
                    }
                    1 => {
                        commands.spawn(MonsterType2::bundle(monster_builder, pos, &assets));
                    }
                    2 => {
                        commands.spawn(MonsterType3::bundle(monster_builder, pos, &assets));
                    }
                    _ => unreachable!(),
                }
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
    monsters: Query<(&MonsterRarity, &MonsterLevel)>,
) {
    let monster_entity = trigger.target();
    commands
        .entity(monster_entity)
        .observe(monster_dying)
        .observe(increment_score);

    // Customize Rare monsters
    let Ok((MonsterRarity::Rare, &MonsterLevel(mlevel))) = monsters.get(monster_entity) else {
        return;
    };

    // All rare monster have more life
    commands.spawn((MoreLife(10.), ChildOf(monster_entity)));

    // Add upgrades depending on map level
    let mut upgrade_provider = UpgradeProvider::new();
    let mut rng = rand::rng();
    for _ in 0..mlevel.saturating_sub(1) {
        if let Some(upgrade) = upgrade_provider.gen(&mut rng) {
            let upgrade_view = upgrade.generate(&mut commands, &mut rng);
            commands
                .entity(upgrade_view.entity)
                .insert(ChildOf(monster_entity));
        }
    }
}

fn customize_monster_type_1(
    trigger: Trigger<OnAdd, MonsterType1>,
    mut commands: Commands,
    monsters: Query<(&MonsterRarity, &MonsterLevel)>,
) {
    let monster_entity = trigger.target();
    let Ok((MonsterRarity::Rare, &MonsterLevel(mlevel))) = monsters.get(monster_entity) else {
        return;
    };

    // Add a wand and the fireball skill
    let mut rng = rand::rng();
    let spawner = ItemSpawner::new(mlevel, &mut rng);
    let weapon = spawner.spawn::<Wand>(&mut commands, &mut rng);
    commands.entity(weapon).insert(ChildOf(monster_entity));
    commands.spawn((FireBallLauncher, ChildOf(monster_entity)));
}

fn customize_monster_type_2(
    trigger: Trigger<OnAdd, MonsterType2>,
    mut commands: Commands,
    monsters: Query<(&MonsterRarity, &MonsterLevel)>,
) {
    let monster_entity = trigger.target();
    let Ok((MonsterRarity::Rare, &MonsterLevel(mlevel))) = monsters.get(monster_entity) else {
        return;
    };

    // Add a wand and the death aura skill
    let mut rng = rand::rng();
    let spawner = ItemSpawner::new(mlevel, &mut rng);
    let weapon = spawner.spawn::<Wand>(&mut commands, &mut rng);
    commands.entity(weapon).insert(ChildOf(monster_entity));
    commands.spawn((DeathAura, ChildOf(monster_entity)));
}

fn customize_monster_type_3(
    trigger: Trigger<OnAdd, MonsterType3>,
    mut commands: Commands,
    monsters: Query<(&MonsterRarity, &MonsterLevel)>,
) {
    let monster_entity = trigger.target();
    let Ok((MonsterRarity::Rare, &MonsterLevel(mlevel))) = monsters.get(monster_entity) else {
        return;
    };

    // Add a wand and the fireball skill
    let mut rng = rand::rng();
    let spawner = ItemSpawner::new(mlevel, &mut rng);
    let weapon = spawner.spawn::<Wand>(&mut commands, &mut rng);
    commands.entity(weapon).insert(ChildOf(monster_entity));
    commands.spawn((ShurikenLauncher, ChildOf(monster_entity)));
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
    mut skills: Query<(Entity, &mut AttackTimer, &ChildOf), With<Skill>>,
) {
    let Ok(player_pos) = players.single().map(|t| t.translation.xy()) else {
        return;
    };
    for (entity, mut timer, &ChildOf(parent)) in &mut skills {
        if let Ok((pos, view_range)) = monsters.get(parent) {
            let distance = (player_pos - pos.translation.xy()).length();
            if timer.finished() && distance <= **view_range {
                commands.trigger(ActivateSkill(entity, player_pos));
                timer.reset();
            }
        }
    }
}
