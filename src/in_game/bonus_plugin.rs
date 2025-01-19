use super::{game_is_running, GameState};
use crate::{
    camera::MainCamera,
    components::*,
    schedule::GameRunningSet,
    utils::picking::{WorldPosition, BONUS_ITEM_DEPTH},
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
use rand::thread_rng;

pub struct BonusPlugin;

impl Plugin for BonusPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bonus>()
            .add_systems(OnExit(GameState::InGame), despawn_all::<Bonus>)
            .add_systems(
                PreUpdate,
                bonus_picking_backend
                    .in_set(PickSet::Backend)
                    .run_if(game_is_running),
            )
            .add_systems(Update, spawn_bonus.in_set(GameRunningSet::EntityUpdate));
    }
}

const BONUS_SCALE: f32 = 0.3;
const BONUS_SIZE: Vec2 = vec2(
    BONUS_ITEM_SIZE.x as f32 * BONUS_SCALE,
    BONUS_ITEM_SIZE.y as f32 * BONUS_SCALE,
);

fn bonus_picking_backend(
    pointers: Query<(&PointerId, &PointerLocation)>,
    camera: Single<(Entity, &Camera, &GlobalTransform), With<MainCamera>>,
    bonuses: Query<(Entity, &GlobalTransform), With<Bonus>>,
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
        for (bonus_entity, transform) in &bonuses {
            let bounds = Rect::from_center_size(transform.translation().xy(), BONUS_SIZE);
            if bounds.contains(pointer_world_pos) {
                picks.push((
                    bonus_entity,
                    HitData::new(camera_entity, BONUS_ITEM_DEPTH, None, None),
                ));
            }
        }
        if !picks.is_empty() {
            // warn!("pointer: {pointer_world_pos} on {:?}", picks);
            output.send(PointerHits::new(*pointer_id, picks, order));
        }
    }
}

fn spawn_bonus(
    mut commands: Commands,
    mut monster_death_events: EventReader<MonsterDeathEvent>,
    assets: Res<EquipmentAssets>,
) {
    let mut rng = thread_rng();
    for event in monster_death_events.read() {
        if let Some(equipment_info) = BonusProvider::spawn(&mut commands, &mut rng) {
            let translation = event.pos.with_z(LAYER_ITEM);
            commands
                .spawn((
                    Bonus(equipment_info.entity),
                    assets.sprite(equipment_info.info.tile_index),
                    Transform::from_translation(translation).with_scale(Vec3::splat(BONUS_SCALE)),
                ))
                .observe(take_bonus);
        }
    }
}

fn take_bonus(
    mut trigger: Trigger<Pointer<Click>>,
    mut player: Single<&mut CharacterAction, With<Player>>,
) {
    player.take_item(trigger.entity());
    trigger.propagate(false);
}
