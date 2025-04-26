use crate::{
    camera::MainCamera,
    components::{
        affix::{IncreaseAreaOfEffect, PierceChance},
        animation::AnimationTimer,
        character::{
            CharacterAction, CharacterDiedEvent, CharacterDyingEvent, CharacterLevel, Life,
            LooseLifeEvent, MaxLife,
        },
        despawn_all,
        equipment::weapon::AttackTimer,
        inventory::{
            Inventory, InventoryChanged, InventoryPos, PlayerEquipmentChanged,
            TakeDroppedItemCommand,
        },
        item::DroppedItem,
        monster::MonsterDeathEvent,
        orb::OrbProvider,
        player::{
            EquipSkillGemCommand, Experience, LevelUpEvent, NextPositionIndicator,
            NextPositionIndicatorAssets, Player, PlayerAction, PlayerAssets, PlayerDeathEvent,
            PlayerSkills, Score,
        },
        skills::{spawn_skill, ActivateSkill, Skill},
        world_map::{WorldMap, WorldMapLoadingFinished, LAYER_PLAYER},
        GROUP_ENEMY,
    },
    schedule::{GameRunningSet, GameState},
    utils::{blink::Blink, invulnerable::Invulnerable},
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAssets>()
            .init_resource::<NextPositionIndicatorAssets>()
            .init_resource::<Score>()
            .add_event::<PlayerDeathEvent>()
            .add_event::<InventoryChanged>()
            .add_event::<PlayerEquipmentChanged>()
            .register_type::<Experience>()
            .register_type::<Inventory>()
            .register_type::<InventoryPos>()
            .register_type::<PlayerSkills>()
            .add_systems(OnEnter(GameState::InGame), (spawn_player, unpause))
            .add_systems(
                OnExit(GameState::InGame),
                (despawn_all::<Player>, despawn_all::<Inventory>, pause),
            )
            .add_systems(
                Update,
                (
                    animate_player_sprite,
                    player_invulnerability_finished,
                    increment_player_experience,
                    refill_life_on_level_up,
                    activate_skill,
                )
                    .in_set(GameRunningSet::EntityUpdate),
            )
            .add_observer(move_player)
            .add_observer(manage_player_movement_with_mouse);
    }
}

fn world_position(
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    pos: Vec2,
) -> Option<Vec2> {
    cameras
        .single()
        .ok()
        .and_then(|(camera, transform)| camera.viewport_to_world_2d(transform, pos).ok())
}

fn manage_player_movement_with_mouse(trigger: Trigger<OnAdd, WorldMap>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .observe(
            |trigger: Trigger<Pointer<Pressed>>,
             mut commands: Commands,
             mut player: Single<&mut CharacterAction, With<Player>>,
             cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
             assets: Res<NextPositionIndicatorAssets>| {
                if let Some(world_pos) =
                    world_position(cameras, trigger.event().pointer_location.position)
                {
                    player.goto(world_pos);
                    commands.spawn((
                        NextPositionIndicator,
                        Mesh2d(assets.mesh.clone()),
                        MeshMaterial2d(assets.color.clone()),
                        Transform::from_translation(world_pos.extend(10.)),
                    ));
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Drag>>,
             mut player: Single<&mut CharacterAction, With<Player>>,
             cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>| {
                if let Some(world_pos) =
                    world_position(cameras, trigger.event().pointer_location.position)
                {
                    player.goto(world_pos);
                }
            },
        );
}

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands.spawn(Inventory::default());

    let player_id = commands
        .spawn((Player, Player::sprite(&assets)))
        .observe(set_invulnerable_on_hit)
        .observe(player_dying)
        .id();

    // Add a skill to the player
    let info = spawn_skill::<crate::components::skills::shuriken::ShurikenLauncher>(&mut commands);
    commands.queue(EquipSkillGemCommand(info.entity, PlayerAction::Skill1));

    // TEMP
    {
        commands.entity(player_id).with_children(|p| {
            p.spawn(IncreaseAreaOfEffect(50.));
            p.spawn(PierceChance(50.));
        });

        let mut rng = rand::rng();
        let orb = OrbProvider::spawn(&mut commands, &mut rng);
        let drop = commands.spawn(DroppedItem(orb.entity)).id();
        commands.queue(TakeDroppedItemCommand(drop));
    }
}

fn move_player(
    trigger: Trigger<WorldMapLoadingFinished>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    player.translation = trigger.event().translation.extend(LAYER_PLAYER);
}

fn pause(mut query: Query<(&mut Invulnerable, &mut Blink), With<Player>>) {
    if let Ok((mut invulnerable, mut blink)) = query.single_mut() {
        invulnerable.pause(true);
        blink.pause(true);
    }
}

fn unpause(mut query: Query<(&mut Invulnerable, &mut Blink), With<Player>>) {
    if let Ok((mut invulnerable, mut blink)) = query.single_mut() {
        invulnerable.pause(false);
        blink.pause(false);
    }
}

fn set_invulnerable_on_hit(
    trigger: Trigger<LooseLifeEvent>,
    mut commands: Commands,
    mut players: Query<&mut CollisionGroups, With<Player>>,
) {
    if let Ok(mut collision_groups) = players.get_mut(trigger.target()) {
        // Set player invulnerable
        commands.entity(trigger.target()).insert((
            Invulnerable::new(Duration::from_secs_f32(1.0), GROUP_ENEMY),
            Blink::new(Duration::from_secs_f32(0.15)),
        ));

        // To allow player to not collide with enemies
        collision_groups.filters &= !GROUP_ENEMY;
    }
}

fn player_dying(
    trigger: Trigger<CharacterDyingEvent>,
    mut commands: Commands,
    mut send_died: EventWriter<CharacterDiedEvent>,
) {
    info!("player_dying");
    commands.trigger(PlayerDeathEvent);
    send_died.write(CharacterDiedEvent(trigger.target()));
}

///
/// Animate the player sprite
///
fn animate_player_sprite(
    time: Res<Time>,
    mut q_player: Query<(&Velocity, &mut AnimationTimer, &mut Sprite), With<Player>>,
) {
    if let Ok((&velocity, mut timer, mut sprite)) = q_player.single_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if velocity == Velocity::zero() {
                    0
                } else {
                    match atlas.index {
                        4 => 8,
                        8 => 12,
                        12 => 16,
                        16 => 4,
                        _ => 4,
                    }
                }
            }
        }
    }
}

fn player_invulnerability_finished(
    mut commands: Commands,
    q_player: Query<(), With<Player>>,
    mut entities: RemovedComponents<Invulnerable>,
) {
    for entity in entities.read() {
        if q_player.get(entity).is_ok() {
            info!("player_invulnerability_finished");
            commands.entity(entity).remove::<Blink>();
        }
    }
}

///
/// Update player XP when monster died
///
fn increment_player_experience(
    mut monster_death_reader: EventReader<MonsterDeathEvent>,
    mut q_player: Query<(&mut Experience, &mut CharacterLevel), With<Player>>,
    mut level_up_sender: EventWriter<LevelUpEvent>,
) {
    if let Ok((mut experience, mut level)) = q_player.single_mut() {
        for monster_death_ev in monster_death_reader.read() {
            experience.add(monster_death_ev.xp);
            let current_level = experience.level();
            if current_level > **level {
                // LEVEL UP !
                **level = current_level;
                info!("Level up : {current_level}");
                level_up_sender.write(LevelUpEvent);
            }
        }
    }
}

fn refill_life_on_level_up(
    mut q_player: Query<(&mut Life, &MaxLife), With<Player>>,
    mut level_up_rcv: EventReader<LevelUpEvent>,
) {
    if let Ok((mut life, max_life)) = q_player.single_mut() {
        for _ in level_up_rcv.read() {
            // Regen life
            life.regenerate(**max_life);
        }
    }
}

fn activate_skill(
    mut commands: Commands,
    players: Query<&PlayerSkills, With<Player>>,
    mut skills: Query<&mut AttackTimer, With<Skill>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    buttons: Res<ButtonInput<KeyCode>>,
) {
    let Ok(player_skills) = players.single() else {
        return;
    };

    let Some(pos) = windows
        .single()
        .ok()
        .and_then(|w| w.cursor_position())
        .and_then(|pos| world_position(cameras, pos))
    else {
        return;
    };

    let mut actions = vec![];
    if buttons.pressed(KeyCode::KeyQ) {
        actions.push(PlayerAction::Skill1);
    }
    if buttons.pressed(KeyCode::KeyW) {
        actions.push(PlayerAction::Skill2);
    }
    if buttons.pressed(KeyCode::KeyE) {
        actions.push(PlayerAction::Skill3);
    }
    if buttons.pressed(KeyCode::KeyR) {
        actions.push(PlayerAction::Skill4);
    }

    for action in actions {
        if let Some(skill) = player_skills.get(action) {
            if let Ok(mut timer) = skills.get_mut(skill) {
                if timer.finished() {
                    commands.trigger(ActivateSkill(skill, pos));
                    timer.reset();
                }
            }
        }
    }
}
