use crate::{
    camera::MainCamera,
    components::{despawn_all, world_map::*},
    schedule::{GameRunningSet, GameState},
    utils::picking::{WorldPosition, MAP_DEPTH},
};
use bevy::{
    picking::{
        backend::{HitData, PointerHits},
        pointer::{PointerId, PointerLocation},
        PickSet,
    },
    prelude::*,
};
use bevy_ecs_tilemap::TilemapPlugin;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .register_type::<MapLevelConfig>()
            .init_resource::<WorldMapAssets>()
            .init_resource::<CurrentMapLevel>()
            .init_resource::<ChangeLevelTimer>()
            .add_systems(OnEnter(GameState::InGame), (reset_level, spawn_worldmap))
            .add_systems(OnExit(GameState::InGame), despawn_all::<WorldMap>)
            .add_systems(
                PreUpdate,
                (
                    // level_selection_follow_player,
                    world_map_picking_backend.in_set(PickSet::Backend),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (spawn_chunks, despawn_out_of_range_chunks, change_level)
                    .in_set(GameRunningSet::EntityUpdate),
                // (spawn_characters, spawn_colliders).in_set(GameRunningSet::EntityUpdate),
            );
    }
}

#[derive(Resource, Deref, DerefMut)]
struct ChangeLevelTimer(Timer);

impl Default for ChangeLevelTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(20.0, TimerMode::Repeating))
    }
}

fn reset_level(mut level: ResMut<CurrentMapLevel>, mut timer: ResMut<ChangeLevelTimer>) {
    *level = CurrentMapLevel::default();
    timer.reset();
}

fn spawn_worldmap(mut commands: Commands) {
    let mut rng = rand::rng();
    let config = WorldMapConfig::default();
    let map = ProceduralWorldMap::new(config, &mut rng);
    commands.insert_resource(map);
    commands.spawn(WorldMap);
}

fn change_level(
    mut level: ResMut<CurrentMapLevel>,
    mut timer: ResMut<ChangeLevelTimer>,
    time: Res<Time>,
) {
    if timer.tick(time.delta()).just_finished() {
        level.next();
    }
}

fn spawn_chunks(
    mut commands: Commands,
    cameras: Query<&Transform, With<MainCamera>>,
    world_maps: Query<Entity, With<WorldMap>>,
    mut world_map: ResMut<ProceduralWorldMap>,
    assets: Res<WorldMapAssets>,
) {
    let Ok(camera_pos) = cameras.single().map(|t| t.translation.xy()) else {
        return;
    };
    let Ok(map_entity) = world_maps.single() else {
        return;
    };
    let camera_chunk_pos = world_map.chunk_pos(camera_pos);
    let mut chunk_entities = Vec::with_capacity(9);
    for y in (camera_chunk_pos.y - 1)..=(camera_chunk_pos.y + 1) {
        for x in (camera_chunk_pos.x - 1)..=(camera_chunk_pos.x + 1) {
            let chunk_pos = IVec2::new(x, y);
            if !world_map.is_spawned(chunk_pos) {
                let chunk_entity = world_map.spawn_chunk(&mut commands, &assets, chunk_pos);
                chunk_entities.push(chunk_entity);
            }
        }
    }
    commands.entity(map_entity).add_children(&chunk_entities);
}

fn despawn_out_of_range_chunks(
    mut commands: Commands,
    cameras: Query<&Transform, With<MainCamera>>,
    chunks_query: Query<(Entity, &Transform), With<WorldMapChunk>>,
    mut world_map: ResMut<ProceduralWorldMap>,
) {
    let Ok(camera_pos) = cameras.single().map(|t| t.translation.xy()) else {
        return;
    };
    for (entity, chunk_transform) in chunks_query.iter() {
        let chunk_pos = chunk_transform.translation.xy();
        let distance = camera_pos.distance(chunk_pos);
        if world_map.remove_chunk_if_out_of_bound(chunk_pos, distance) {
            commands.entity(entity).despawn();
        }
    }
}

fn world_map_picking_backend(
    pointers: Query<(&PointerId, &PointerLocation)>,
    camera: Single<(Entity, &Camera, &GlobalTransform), With<MainCamera>>,
    worlds_maps: Query<Entity, With<WorldMap>>,
    mut output: EventWriter<PointerHits>,
) {
    let Ok(world_map) = worlds_maps.single() else {
        return;
    };
    let (camera_entity, camera, camera_transform) = *camera;
    for (pointer_id, pointer_location) in &pointers {
        let Some(pointer_world_pos) = pointer_location.world_position(camera, camera_transform)
        else {
            continue;
        };

        let depth = MAP_DEPTH;
        let position = Some(pointer_world_pos.extend(0.));
        let picks = vec![(
            world_map,
            HitData::new(camera_entity, depth, position, None),
        )];
        let order = camera.order as f32;
        output.write(PointerHits::new(*pointer_id, picks, order));
    }
}
