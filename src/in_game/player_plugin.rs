use crate::{
    camera::MainCamera,
    components::{
        animation::AnimationTimer,
        character::{
            CharacterAction, CharacterDiedEvent, CharacterDyingEvent, CharacterLevel, Life,
            LooseLifeEvent, MaxLife,
        },
        despawn_all,
        equipment::{weapon::AttackTimer, Equipment},
        inventory::{
            AddToInventoryEvent, Inventory, InventoryChanged, InventoryPos, PlayerEquipmentChanged,
            RemoveFromInventoryEvent, TakeDroppedItemEvent,
        },
        item::{DroppedItem, EquipEquipmentEvent, Item},
        monster::MonsterDeathEvent,
        player::{
            EquipSkillBookEvent, Experience, LevelUpEvent, NextPositionIndicator,
            NextPositionIndicatorAssets, Player, PlayerAction, PlayerAssets, PlayerBooks,
            PlayerDeathEvent, Score,
        },
        skills::{
            shuriken::ShurikenLauncherBook, spawn_book, ActivateSkill, AssociatedSkill, Skill,
            SkillBook,
        },
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
            .register_type::<PlayerBooks>()
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
            .add_observer(manage_player_movement_with_mouse)
            .add_observer(equip_equipment)
            .add_observer(equip_skill_book)
            .add_observer(take_dropped_item)
            .add_observer(add_to_inventory)
            .add_observer(remove_from_inventory);
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
                if let Some(world_pos) = world_position(cameras, trigger.pointer_location.position)
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
                if let Some(world_pos) = world_position(cameras, trigger.pointer_location.position)
                {
                    player.goto(world_pos);
                }
            },
        );
}

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands.spawn(Inventory::default());
    commands
        .spawn((Player, Player::sprite(&assets)))
        .observe(set_invulnerable_on_hit)
        .observe(player_dying);

    // Add a skill to the player
    let info = spawn_book::<ShurikenLauncherBook>(&mut commands);
    commands.trigger(EquipSkillBookEvent {
        book_entity: info.entity,
        action: PlayerAction::Skill1,
    });
}

fn move_player(
    trigger: Trigger<WorldMapLoadingFinished>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    player.translation = trigger.translation.extend(LAYER_PLAYER);
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
    players: Query<&PlayerBooks, With<Player>>,
    books: Query<&AssociatedSkill, With<SkillBook>>,
    mut skills: Query<&mut AttackTimer, With<Skill>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    buttons: Res<ButtonInput<KeyCode>>,
) {
    let Ok(player_books) = players.single() else {
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
        if let Some(book) = player_books.get(action) {
            if let Ok(&AssociatedSkill(skill)) = books.get(book) {
                if let Ok(mut timer) = skills.get_mut(skill) {
                    if timer.finished() {
                        commands.trigger(ActivateSkill(skill, pos));
                        timer.reset();
                    }
                }
            }
        }
    }
}

fn equip_equipment(
    trigger: Trigger<EquipEquipmentEvent>,
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    equipments: Query<(Entity, &Equipment, &ChildOf)>,
) {
    let Ok(equipment_to_equip) = equipments.get(trigger.0).map(|(_, eqp, _)| *eqp) else {
        warn!("Can't equip {} as it's not an Equipment", trigger.0);
        return;
    };

    // Check it the player already have an item of same type
    let player = players.single().expect("Player");
    let old_equipment = equipments
        .iter()
        // same parent, same type, but different entity
        .filter(|(entity, eqp, child_of)| {
            player == child_of.parent() && **eqp == equipment_to_equip && *entity != trigger.0
        })
        .map(|(e, _eqp, _p)| e)
        // There should be at most 1 equipment
        .next();

    // Manage inventory
    commands.trigger(RemoveFromInventoryEvent(trigger.0));
    if let Some(old_equipment) = old_equipment {
        commands.trigger(AddToInventoryEvent::new(old_equipment));
    }

    info!("Equip equipment {}", trigger.0);

    commands.entity(player).add_child(trigger.0);
    commands.trigger(PlayerEquipmentChanged);
}

fn equip_skill_book(
    trigger: Trigger<EquipSkillBookEvent>,
    mut commands: Commands,
    books: Query<(), With<SkillBook>>,
    mut players: Query<(Entity, &mut PlayerBooks), With<Player>>,
) {
    if !books.contains(trigger.book_entity) {
        warn!(
            "Can't equip {} as it's not an SkillBook",
            trigger.book_entity
        );
        return;
    };

    let (player_entity, mut player_books) = players
        .single_mut()
        .expect("Player should have a PlayerBooks");

    let old_book = match player_books.get(trigger.action) {
        Some(old_book) => {
            if old_book == trigger.book_entity {
                // same gem: no need to continue
                return;
            }
            player_books.remove(old_book);
            Some(old_book)
        }
        None => None,
    };
    player_books.remove(trigger.book_entity);
    player_books.set_book(trigger.action, trigger.book_entity);

    // Manage inventory
    commands.trigger(RemoveFromInventoryEvent(trigger.book_entity));
    if let Some(old_book) = old_book {
        commands.trigger(AddToInventoryEvent::new(old_book));
    }

    info!(
        "Equip skill book {} to {:?}",
        trigger.book_entity, trigger.action
    );

    commands
        .entity(player_entity)
        .add_child(trigger.book_entity);
    commands.trigger(PlayerEquipmentChanged);
}

fn take_dropped_item(
    trigger: Trigger<TakeDroppedItemEvent>,
    mut commands: Commands,
    dropped_items: Query<&DroppedItem>,
    mut inventories: Query<(Entity, &mut Inventory)>,
) {
    let dropped_item_entity = trigger.0;
    let Ok(dropped_item) = dropped_items.get(dropped_item_entity).cloned() else {
        warn!("Can't take item from {dropped_item_entity} as it's not a [DroppedItem]");
        return;
    };
    let Ok((inventory_entity, mut inventory)) = inventories.single_mut() else {
        error!("Inventory doesn't exist!");
        return;
    };

    let item_entity = *dropped_item;
    if inventory.add(item_entity) {
        info!("Take dropped item {dropped_item_entity} => {item_entity}");
        commands.entity(inventory_entity).add_child(item_entity);
        commands.entity(dropped_item_entity).despawn();
        commands.trigger(InventoryChanged);
    }
}

fn add_to_inventory(
    trigger: Trigger<AddToInventoryEvent>,
    mut commands: Commands,
    mut inventories: Query<(Entity, &mut Inventory)>,
    mut player_skills: Query<&mut PlayerBooks>,
) {
    let Ok((inventory_entity, mut inventory)) = inventories.single_mut() else {
        error!("Inventory doesn't exist!");
        return;
    };

    // Allow to move an item
    inventory.remove(trigger.item);
    let added = match trigger.pos {
        Some(pos) => inventory.add_at(trigger.item, pos),
        None => inventory.add(trigger.item),
    };

    if added {
        info!("Add item {} to inventory", trigger.item);
        commands
            .entity(trigger.item)
            .insert(ChildOf(inventory_entity));

        // remove from skill if it was a skill
        // TODO: probably WRONG!
        player_skills
            .single_mut()
            .expect("PlayerSkills")
            .remove(trigger.item);

        commands.trigger(InventoryChanged);
    }
}

fn remove_from_inventory(
    trigger: Trigger<RemoveFromInventoryEvent>,
    mut commands: Commands,
    mut inventories: Query<(Entity, &mut Inventory)>,
    items: Query<&ChildOf, With<Item>>,
) {
    let Ok((inventory_entity, mut inventory)) = inventories.single_mut() else {
        error!("Inventory doesn't exist!");
        return;
    };
    let item = trigger.0;
    if inventory.remove(item) {
        info!("Remove item {item} from inventory");
        commands.entity(inventory_entity).remove::<InventoryPos>();
        if let Ok(&ChildOf(parent)) = items.get(item) {
            if inventory_entity == parent {
                warn!("Remove item {item} from inventory : removing ChildOf");
                commands.entity(item).remove::<ChildOf>();
            }
        }
        commands.trigger(InventoryChanged);
    }
}
