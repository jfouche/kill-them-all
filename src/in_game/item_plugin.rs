use super::dnd::{DndCursor, DraggedEntity};
use crate::{
    camera::MainCamera,
    components::{
        character::CharacterAction,
        despawn_all,
        inventory::{Inventory, PlayerEquipmentChanged, RemoveFromInventoryEvent},
        item::{
            DropItemEvent, DroppedItem, Item, ItemAssets, ItemInfo, ItemLevel, ItemProvider,
            ItemRarity, ITEM_SIZE,
        },
        monster::MonsterDeathEvent,
        player::{Player, RemoveSkillBookEvent},
        world_map::{WorldMap, LAYER_ITEM},
    },
    schedule::{game_is_running, GameRunningSet, GameState},
    utils::picking::{WorldPosition, ITEM_DEPTH},
};
use bevy::{
    math::vec2,
    picking::{
        backend::{HitData, PointerHits},
        pointer::{PointerId, PointerLocation},
        PickSet,
    },
    prelude::*,
};
use rand::Rng;
use std::f32::consts::PI;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ItemRarity>()
            .register_type::<ItemLevel>()
            .register_type::<DroppedItem>()
            .register_type::<ItemInfo>()
            .add_systems(
                OnExit(GameState::InGame),
                (despawn_all::<DroppedItem>, despawn_all::<Item>),
            )
            .add_systems(
                PreUpdate,
                item_picking_backend
                    .in_set(PickSet::Backend)
                    .run_if(game_is_running),
            )
            .add_systems(
                Update,
                drop_item_on_monster_death.in_set(GameRunningSet::EntityUpdate),
            )
            .add_observer(update_player_on_drop_item)
            .add_observer(
                |trigger: Trigger<OnAdd, WorldMap>, mut commands: Commands| {
                    commands.entity(trigger.target()).observe(player_drop_item);
                },
            );
    }
}

const ITEM_WORLD_SCALE: f32 = 0.3;
const ITEM_WORLD_SIZE: Vec2 = vec2(
    ITEM_SIZE.x as f32 * ITEM_WORLD_SCALE,
    ITEM_SIZE.y as f32 * ITEM_WORLD_SCALE,
);

fn item_picking_backend(
    pointers: Query<(&PointerId, &PointerLocation)>,
    camera: Single<(Entity, &Camera, &GlobalTransform), With<MainCamera>>,
    items: Query<(Entity, &GlobalTransform), With<DroppedItem>>,
    mut output: EventWriter<PointerHits>,
) {
    let (camera_entity, camera, camera_transform) = *camera;
    let order = camera.order as f32;
    for (pointer_id, pointer_location) in &pointers {
        let mut picks = Vec::new();
        let Some(pointer_world_pos) = pointer_location.world_position(camera, camera_transform)
        else {
            continue;
        };
        for (item_entity, transform) in &items {
            let bounds = Rect::from_center_size(transform.translation().xy(), ITEM_WORLD_SIZE);
            if bounds.contains(pointer_world_pos) {
                picks.push((
                    item_entity,
                    HitData::new(camera_entity, ITEM_DEPTH, None, None),
                ));
            }
        }
        if !picks.is_empty() {
            output.write(PointerHits::new(*pointer_id, picks, order));
        }
    }
}

fn drop_item_on_monster_death(
    mut commands: Commands,
    mut monster_death_events: EventReader<MonsterDeathEvent>,
    assets: Res<ItemAssets>,
) {
    let mut rng = rand::rng();
    for event in monster_death_events.read() {
        let provider = ItemProvider(event.mlevel);
        if let Some(item_info) = provider.spawn(&mut commands, &mut rng) {
            let translation = event.pos.with_z(LAYER_ITEM);
            commands
                .spawn((
                    DroppedItem(item_info.entity),
                    assets.sprite(item_info.info.tile_index),
                    Transform::from_translation(translation)
                        .with_scale(Vec3::splat(ITEM_WORLD_SCALE)),
                ))
                .observe(take_dropped_item);
        }
    }
}

fn update_player_on_drop_item(
    trigger: Trigger<DropItemEvent>,
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    inventories: Query<Entity, With<Inventory>>,
    items: Query<&ChildOf, With<Item>>,
) {
    let player = players.single().expect("Player");
    let inventory = inventories.single().expect("Inventory");

    enum Change {
        Player,
        Inventory,
        Other,
    }

    let item = trigger.0;
    let Ok(change) = items.get(item).map(|&ChildOf(parent)| {
        if parent == player {
            Change::Player
        } else if parent == inventory {
            Change::Inventory
        } else {
            Change::Other
        }
    }) else {
        // No parent
        return;
    };

    match change {
        Change::Player => {
            commands.entity(item).remove::<ChildOf>();
        }
        Change::Inventory => {
            commands.trigger(RemoveFromInventoryEvent(item));
        }
        Change::Other => {
            commands.entity(item).remove::<ChildOf>();
        }
    }
    commands.entity(item).despawn();

    if let Change::Player = change {
        commands.trigger(PlayerEquipmentChanged);
    }
}

pub fn take_dropped_item(
    mut trigger: Trigger<Pointer<Click>>,
    mut player: Single<&mut CharacterAction, With<Player>>,
) {
    player.take_item(trigger.target());
    trigger.propagate(false);
}

fn player_drop_item(
    _trigger: Trigger<Pointer<DragDrop>>,
    mut commands: Commands,
    cursors: Query<&DraggedEntity, With<DndCursor>>,
    items: Query<&ItemInfo, With<Item>>,
    players: Query<&Transform, With<Player>>,
    assets: Res<ItemAssets>,
) {
    let Ok(&DraggedEntity(Some(item))) = cursors.single() else {
        error!("No entity dragged");
        return;
    };

    let Ok(item_info) = items.get(item) else {
        error!("Item {item} doesn't have ItemInfo");
        return;
    };
    info!("player_drop_item({item})");

    // Remove it from player
    commands.trigger(RemoveFromInventoryEvent(item));
    commands.trigger(RemoveSkillBookEvent { book_entity: item });
    commands.entity(item).try_remove::<ChildOf>();
    commands.trigger(PlayerEquipmentChanged);

    // Spawn the DroppedItem next to the player
    let mut rng = rand::rng();
    let player_pos = players.single().expect("Player").translation;
    let dist = rng.random_range(5..10) as f32;
    let angle = rng.random_range(0. ..(2. * PI));
    let pos = Vec3 {
        x: player_pos.x + dist * angle.cos(),
        y: player_pos.y + dist * angle.sin(),
        z: LAYER_ITEM,
    };
    commands
        .spawn((
            DroppedItem(item),
            assets.sprite(item_info.tile_index),
            Transform::from_translation(pos).with_scale(Vec3::splat(ITEM_WORLD_SCALE)),
        ))
        .observe(take_dropped_item);
}
