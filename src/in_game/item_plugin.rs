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
        player::Player,
        world_map::LAYER_ITEM,
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
                spawn_dropped_item.in_set(GameRunningSet::EntityUpdate),
            )
            .add_observer(drop_item);
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
            // warn!("pointer: {pointer_world_pos} on {:?}", picks);
            output.write(PointerHits::new(*pointer_id, picks, order));
        }
    }
}

fn spawn_dropped_item(
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

fn drop_item(
    trigger: Trigger<DropItemEvent>,
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    inventories: Query<Entity, With<Inventory>>,
    items: Query<&ChildOf, With<Item>>,
) {
    let player = players.single().expect("Player");
    let inventory = inventories.single().expect("Inventory");

    enum Change {
        None,
        Player,
        Inventory,
        Other(Entity),
    }

    let change = items
        .get(trigger.0)
        .map(|child_of| {
            let parent = child_of.parent();
            if parent == player {
                Change::Player
            } else if parent == inventory {
                Change::Inventory
            } else {
                Change::Other(parent)
            }
        })
        .unwrap_or(Change::None);

    match change {
        Change::Player => {
            commands.entity(player).remove_children(&[trigger.0]);
        }
        Change::Inventory => {
            commands.trigger(RemoveFromInventoryEvent(trigger.0));
        }
        Change::Other(parent) => {
            commands.entity(parent).remove_children(&[trigger.0]);
        }
        Change::None => {}
    }
    commands.entity(trigger.0).despawn();

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
