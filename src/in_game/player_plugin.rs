use crate::components::*;
use crate::schedule::*;
use crate::utils::blink::Blink;
use crate::utils::invulnerable::Invulnerable;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::ops::Mul;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAssets>()
            .init_resource::<Score>()
            .add_event::<PlayerDeathEvent>()
            .add_event::<InventoryChanged>()
            .add_event::<PlayerEquipmentChanged>()
            .register_type::<Experience>()
            .add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(
                OnExit(GameState::InGame),
                (despawn_all::<Player>, despawn_all::<Inventory>),
            )
            .add_systems(OnEnter(GameState::InGame), unpause)
            .add_systems(OnExit(GameState::InGame), pause)
            .add_systems(
                Update,
                (
                    player_movement,
                    animate_player_sprite,
                    player_invulnerability_finished,
                    increment_player_experience,
                    level_up,
                    // remove_old_equipment::<Amulet>,
                    // remove_old_equipment::<BodyArmour>,
                    // remove_old_equipment::<Boots>,
                    // remove_old_equipment::<Helmet>,
                    // remove_old_equipment::<Wand>,
                    // remove_old_equipment::<FireBallLauncher>,
                    // remove_old_equipment::<ShurikenLauncher>,
                    // remove_old_equipment::<MineDropper>,
                    // remove_old_equipment::<DeathAura>,
                )
                    .in_set(GameRunningSet::EntityUpdate),
            )
            // .add_observer(inventory_modified::<OnAdd, Parent>)
            // .add_observer(inventory_modified::<OnRemove, Parent>)
            // .add_observer(player_modified::<OnAdd, Parent>)
            // .add_observer(player_modified::<OnRemove, Parent>)
            ;
    }
}

#[derive(Component, Deref, DerefMut)]
struct InvulnerabilityAnimationTimer(Timer);

impl Default for InvulnerabilityAnimationTimer {
    fn default() -> Self {
        InvulnerabilityAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands
        .spawn((Player, Player::sprite(&assets)))
        .with_children(|player| {
            player.spawn(DeathAura);
            player.spawn(IncreaseAreaOfEffect(50.));
        })
        .observe(set_invulnerable_on_hit)
        .observe(player_dying);

    commands.spawn(Inventory);
}

fn pause(mut query: Query<(&mut Invulnerable, &mut Blink), With<Player>>) {
    if let Ok((mut invulnerable, mut blink)) = query.get_single_mut() {
        invulnerable.pause(true);
        blink.pause(true);
    }
}

fn unpause(mut query: Query<(&mut Invulnerable, &mut Blink), With<Player>>) {
    if let Ok((mut invulnerable, mut blink)) = query.get_single_mut() {
        invulnerable.pause(false);
        blink.pause(false);
    }
}

///
/// Manage the keyboard to move the player
///
fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&MovementSpeed, &mut Velocity), With<Player>>,
) {
    if let Ok((speed, mut velocity)) = players.get_single_mut() {
        let mut linvel = Vec2::default();
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::Numpad4, KeyCode::KeyA]) {
            linvel.x = -1.0;
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::Numpad6, KeyCode::KeyD]) {
            linvel.x = 1.0;
        }
        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::Numpad8, KeyCode::KeyW]) {
            linvel.y = 1.0;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::Numpad2, KeyCode::KeyS]) {
            linvel.y = -1.0;
        }
        velocity.linvel = linvel.normalize_or_zero().mul(**speed);
    }
}

fn set_invulnerable_on_hit(
    trigger: Trigger<LooseLifeEvent>,
    mut commands: Commands,
    mut players: Query<&mut CollisionGroups, With<Player>>,
) {
    if let Ok(mut collision_groups) = players.get_mut(trigger.entity()) {
        // Set player invulnerable
        commands
            .entity(trigger.entity())
            .insert(Invulnerable::new(Duration::from_secs_f32(2.0), GROUP_ENEMY))
            .insert(Blink::new(Duration::from_secs_f32(0.15)));

        // To allow player to not collide with enemies
        collision_groups.filters &= !GROUP_ENEMY;
    }
}

fn player_dying(
    trigger: Trigger<CharacterDyingEvent>,
    mut send_died: EventWriter<CharacterDiedEvent>,
    mut send_death: EventWriter<PlayerDeathEvent>,
) {
    info!("send_player_death_event");
    send_died.send(CharacterDiedEvent(trigger.entity()));
    send_death.send(PlayerDeathEvent);
}

///
/// Animate the player sprite
///
fn animate_player_sprite(
    time: Res<Time>,
    mut q_player: Query<(&Velocity, &mut AnimationTimer, &mut Sprite), With<Player>>,
) {
    if let Ok((&velocity, mut timer, mut sprite)) = q_player.get_single_mut() {
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
    mut q_player: Query<&mut Experience, With<Player>>,
    mut level_up_sender: EventWriter<LevelUpEvent>,
) {
    if let Ok(mut experience) = q_player.get_single_mut() {
        for monster_death_ev in monster_death_reader.read() {
            info!("increment_player_experience");
            let level_before = experience.level();
            experience.add(monster_death_ev.xp);
            if experience.level() > level_before {
                // LEVEL UP !
                level_up_sender.send(LevelUpEvent);
            }
        }
    }
}

fn level_up(
    mut q_player: Query<(&mut Life, &MaxLife), With<Player>>,
    mut level_up_rcv: EventReader<LevelUpEvent>,
) {
    if let Ok((mut life, max_life)) = q_player.get_single_mut() {
        for _ in level_up_rcv.read() {
            info!("level_up");
            // Regen life
            life.regenerate(**max_life);
        }
    }
}

// /// When an equipment is added to the player, the old same one should go back to the [Inventory]
// /// TODO: move to observer
// /// FIXME: There is a bug when equiping multiple time the type of equipment
// fn remove_old_equipment<E>(
//     mut commands: Commands,
//     player: Single<Entity, With<Player>>,
//     new_equipments: Query<(Entity, &Parent), (With<E>, Added<Parent>)>,
//     equipments: Query<(Entity, &Parent), With<E>>,
//     inventory: Single<Entity, With<Inventory>>,
// ) where
//     E: Component,
// {
//     for (new_equipment, parent) in &new_equipments {
//         warn!("Equipment {new_equipment} has been added to {}", **parent);
//         if *player == **parent {
//             warn!("Equipment {new_equipment} has been added to Player");
//             // new_equipment has been added to Player, get old ones
//             let old_equipments = equipments
//                 .iter()
//                 // same parent, but different entity
//                 .filter(|(e, p)| {
//                     warn!(" * filter({e}, {})", ***p);
//                     *player == ***p && *e != new_equipment})
//                 .map(|(e, _p)| e)
//                 .collect::<Vec<_>>();

//             if !old_equipments.is_empty() {
//                 // Move old equipments to [Inventory], removing them from [Player]
//                 warn!("Moving back items to inventory: {:?}", &old_equipments);
//                 for equipment in &old_equipments {
//                     commands.entity(*equipment).remove_parent();
//                 }
//                 commands.entity(*player).remove_children(&old_equipments);
//                 commands.entity(*inventory).add_children(&old_equipments);
//             }
//         }
//     }
// }

// fn inventory_modified<E, B>(
//     trigger: Trigger<E, B>,
//     mut commands: Commands,
//     inventory: Single<Entity, With<Inventory>>,
//     parents: Query<&Parent>,
// ) where
//     B: Bundle,
//     E: std::fmt::Debug,
// {
//     if let Ok(parent) = parents.get(trigger.entity()) {
//         if **parent == *inventory {
//             warn!(
//                 "inventory_modified due to {:?}({})",
//                 trigger.event(),
//                 trigger.entity()
//             );
//             commands.trigger(InventoryChanged);
//         }
//     }
// }

// fn player_modified<E, B>(
//     trigger: Trigger<E, B>,
//     mut commands: Commands,
//     player: Single<Entity, With<Player>>,
//     parents: Query<&Parent>,
// ) where
//     B: Bundle,
//     E: std::fmt::Debug,
// {
//     if let Ok(parent) = parents.get(trigger.entity()) {
//         if **parent == *player {
//             warn!(
//                 "player_modified due to {:?}({})",
//                 trigger.event(),
//                 trigger.entity()
//             );
//             commands.trigger(PlayerEquipmentChanged);
//         }
//     }
// }
