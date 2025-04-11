use crate::{
    camera::MainCamera,
    components::{despawn_all, player::Player, world_map::*},
    schedule::{GameRunningSet, GameState},
    utils::picking::{WorldPosition, MAP_DEPTH},
};
use bevy::{
    math::vec2,
    picking::{
        backend::{HitData, PointerHits},
        pointer::{PointerId, PointerLocation},
        PickSet,
    },
    prelude::*,
    utils::HashMap,
};
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin).register_type::<MapLevelConfig>()
            .init_resource::<WorldMapAssets>()
            .init_resource::<CurrentMapLevel>()
            .add_systems(
                OnEnter(GameState::InGame),
                (spawn_worldmap, spawn_characters).chain(),
            )
            .add_systems(OnExit(GameState::InGame), despawn_all::<WorldMap>)
            .add_systems(
                PreUpdate,
                (
                    // level_selection_follow_player,
                    spawn_chunks,
                    world_map_picking_backend.in_set(PickSet::Backend),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            // .add_systems(
            //     Update,
            //     (spawn_characters, spawn_colliders).in_set(GameRunningSet::EntityUpdate),
            // )
            ;
    }
}

fn spawn_worldmap(mut commands: Commands, assets: Res<WorldMapAssets>) {
    // commands.insert_resource(LevelSelection::index(0));
    // commands.spawn((
    //     WorldMap,
    //     LdtkWorldBundle {
    //         ldtk_handle: assets.ldtk_project.clone(),
    //         ..Default::default()
    //     },
    // ));

    let mut rng = rand::rng();
    let map = ProceduralWorldMap::generate(WorldMapConfig::default(), &mut rng);
    map.spawn(&mut commands, &assets);
    commands.insert_resource(map);
    commands.insert_resource(ChunkManager::default());
}

fn spawn_characters(
    mut commands: Commands,
    // mut events: EventReader<LevelEvent>,
    // levels: Query<(Entity, &LevelIid)>,
    // parents: Query<&Parent>,
    // players: Query<(Entity, &GlobalTransform), With<PlayerInitialPosition>>,
    // monsters: Query<(Entity, &GlobalTransform, &MonsterCount), With<MonsterInitialPosition>>,
    configs: Query<(Entity, &MapLevelConfig)>,
    world_map: Res<ProceduralWorldMap>,
) {
    commands.trigger(SpawnMonstersEvent {
        mlevel: 1,
        monsters: vec![(world_map.pos_to_world(5, 5), 3)],
    });

    // for level_entity in events.read().filter_map(|e| {
    //     if let LevelEvent::Transformed(liid) = e {
    //         levels.iter().find(|(_, l)| *l == liid).map(|(e, _)| e)
    //     } else {
    //         None
    //     }
    // }) {
    //     if let Ok((entity, transform)) = players.get_single() {
    //         if parents.iter_ancestors(entity).any(|e| e == level_entity) {
    //             commands.trigger(WorldMapLoadingFinished {
    //                 translation: transform.translation().xy(),
    //             });
    //             commands.entity(entity).remove_parent().despawn();
    //         }
    //     }

    //     let mut monsters_to_spawn = Vec::new();
    //     for (entity, transform, count) in &monsters {
    //         if parents.iter_ancestors(entity).any(|e| e == level_entity) {
    //             monsters_to_spawn.push((transform.translation().xy(), **count));
    //             commands.entity(entity).remove_parent().despawn();
    //         }
    //     }
    //     if !monsters.is_empty() {
    //         let mlevel = configs
    //             .iter()
    //             .find_map(|(entity, config)| {
    //                 parents
    //                     .iter_ancestors(entity)
    //                     .find(|p| *p == level_entity)
    //                     .map(|_| config.monster_level)
    //             })
    //             .expect("A LevelConfig should be present for each level");
    //         commands.trigger(SpawnMonstersEvent {
    //             mlevel,
    //             monsters: monsters_to_spawn,
    //         });
    //     }
    // }
}

fn spawn_chunks(
    mut commands: Commands,
    cameras: Query<&Transform, With<MainCamera>>,
    world_map: Res<ProceduralWorldMap>,
    chunks_mgr: ResMut<ChunkManager>,
) {
    let Ok(camera_pos) = cameras.get_single().map(|t| t.translation.xy()) else {
        return;
    };

    // world_map.tilemap_chunk(camera_pos);

    // let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
    // for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
    //     for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
    //         if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
    //             chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
    //             spawn_chunk(&mut commands, &asset_server, IVec2::new(x, y));
    //         }
    //     }
    // }
}

// /// Spawns colliders of a level
// ///
// /// You could just insert a ColliderBundle into the WallBundle,
// /// but this spawns a different collider for EVERY wall tile.
// /// This approach leads to bad performance.
// ///
// /// Instead, by flagging the wall tiles and spawning the collisions later,
// /// we can minimize the amount of colliding entities.
// ///
// /// The algorithm used here is a nice compromise between simplicity, speed,
// /// and a small number of rectangle colliders.
// /// In basic terms, it will:
// /// 1. consider where the walls are
// /// 2. combine wall tiles into flat "plates" in each individual row
// /// 3. combine the plates into rectangles across multiple rows wherever possible
// /// 4. spawn colliders for each rectangle
// fn spawn_colliders(
//     mut commands: Commands,
//     added_colliders: Query<(&GridCoords, &Parent), Added<ColliderTile>>,
//     parents: Query<&Parent, Without<ColliderTile>>,
//     levels: Query<(Entity, &LevelIid)>,
//     ldtk_projects: Query<&LdtkProjectHandle>,
//     ldtk_project_assets: Res<Assets<LdtkProject>>,
// ) {
//     /// Represents a wide wall that is 1 tile tall
//     /// Used to spawn wall collisions
//     #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
//     struct Plate {
//         left: i32,
//         right: i32,
//     }

//     /// A simple rectangle type representing a wall of any size
//     struct Rect {
//         left: i32,
//         right: i32,
//         top: i32,
//         bottom: i32,
//     }

//     // Consider where the walls are
//     // storing them as GridCoords in a HashSet for quick, easy lookup
//     //
//     // The key of this map will be the entity of the level the wall belongs to.
//     // This has two consequences in the resulting collision entities:
//     // 1. it forces the walls to be split along level boundaries
//     // 2. it lets us easily add the collision entities as children of the appropriate level entity
//     let mut level_to_collider_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

//     added_colliders.iter().for_each(|(&grid_coords, parent)| {
//         // An intgrid tile's direct parent will be a layer entity, not the level entity
//         // To get the level entity, you need the tile's grandparent.
//         // This is where parent_query comes in.
//         if let Ok(grandparent) = parents.get(**parent) {
//             level_to_collider_locations
//                 .entry(grandparent.get())
//                 .or_default()
//                 .insert(grid_coords);
//         }
//     });

//     if !added_colliders.is_empty() {
//         levels.iter().for_each(|(level_entity, level_iid)| {
//             if let Some(level_colliders) = level_to_collider_locations.get(&level_entity) {
//                 let ldtk_project = ldtk_project_assets
//                     .get(ldtk_projects.single())
//                     .expect("Project should be loaded if level has spawned");

//                 let level = ldtk_project
//                     .as_standalone()
//                     .get_loaded_level_by_iid(&level_iid.to_string())
//                     .expect("Spawned level should exist in LDtk project");

//                 let LayerInstance {
//                     c_wid: width,
//                     c_hei: height,
//                     grid_size,
//                     ..
//                 } = level.layer_instances()[0];

//                 // combine wall tiles into flat "plates" in each individual row
//                 let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

//                 for y in 0..height {
//                     let mut row_plates: Vec<Plate> = Vec::new();
//                     let mut plate_start = None;

//                     // + 1 to the width so the algorithm "terminates" plates that touch the right edge
//                     for x in 0..width + 1 {
//                         match (plate_start, level_colliders.contains(&GridCoords { x, y })) {
//                             (Some(s), false) => {
//                                 row_plates.push(Plate {
//                                     left: s,
//                                     right: x - 1,
//                                 });
//                                 plate_start = None;
//                             }
//                             (None, true) => plate_start = Some(x),
//                             _ => (),
//                         }
//                     }

//                     plate_stack.push(row_plates);
//                 }

//                 // combine "plates" into rectangles across multiple rows
//                 let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
//                 let mut prev_row: Vec<Plate> = Vec::new();
//                 let mut collider_rects: Vec<Rect> = Vec::new();

//                 // an extra empty row so the algorithm "finishes" the rects that touch the top edge
//                 plate_stack.push(Vec::new());

//                 for (y, current_row) in plate_stack.into_iter().enumerate() {
//                     for prev_plate in &prev_row {
//                         if !current_row.contains(prev_plate) {
//                             // remove the finished rect so that the same plate in the future starts a new rect
//                             if let Some(rect) = rect_builder.remove(prev_plate) {
//                                 collider_rects.push(rect);
//                             }
//                         }
//                     }
//                     for plate in &current_row {
//                         rect_builder
//                             .entry(plate.clone())
//                             .and_modify(|e| e.top += 1)
//                             .or_insert(Rect {
//                                 bottom: y as i32,
//                                 top: y as i32,
//                                 left: plate.left,
//                                 right: plate.right,
//                             });
//                     }
//                     prev_row = current_row;
//                 }

//                 commands.entity(level_entity).with_children(|level| {
//                     // Spawn colliders for every rectangle..
//                     // Making the collider a child of the level serves two purposes:
//                     // 1. Adjusts the transforms to be relative to the level for free
//                     // 2. the colliders will be despawned automatically when levels unload
//                     for collider_rect in collider_rects {
//                         level.spawn((
//                             MapCollider,
//                             Collider::cuboid(
//                                 (collider_rect.right as f32 - collider_rect.left as f32 + 1.)
//                                     * grid_size as f32
//                                     / 2.,
//                                 (collider_rect.top as f32 - collider_rect.bottom as f32 + 1.)
//                                     * grid_size as f32
//                                     / 2.,
//                             ),
//                             Transform::from_xyz(
//                                 (collider_rect.left + collider_rect.right + 1) as f32
//                                     * grid_size as f32
//                                     / 2.,
//                                 (collider_rect.bottom + collider_rect.top + 1) as f32
//                                     * grid_size as f32
//                                     / 2.,
//                                 0.,
//                             ),
//                         ));
//                     }
//                 });
//             }
//         });
//     }
// }

// fn level_selection_follow_player(
//     players: Query<&GlobalTransform, With<Player>>,
//     levels: Query<(Entity, &LevelIid, &GlobalTransform)>,
//     ldtk_projects: Query<&LdtkProjectHandle>,
//     configs: Query<(Entity, &MapLevelConfig)>,
//     parents: Query<&Parent>,
//     ldtk_project_assets: Res<Assets<LdtkProject>>,
//     mut current_map_level: ResMut<CurrentMapLevel>,
// ) {
//     if let Ok(player_transform) = players.get_single() {
//         let Some(ldtk_project) = ldtk_projects
//             .get_single()
//             .ok()
//             .map(|p| ldtk_project_assets.get(p))
//             .flatten()
//         else {
//             return;
//         };

//         for (level_entity, level_iid, level_transform) in levels.iter() {
//             let level = ldtk_project
//                 .get_raw_level_by_iid(level_iid.get())
//                 .expect("level should exist in only project");

//             let level_bounds = Rect {
//                 min: Vec2::new(
//                     level_transform.translation().x,
//                     level_transform.translation().y,
//                 ),
//                 max: Vec2::new(
//                     level_transform.translation().x + level.px_wid as f32,
//                     level_transform.translation().y + level.px_hei as f32,
//                 ),
//             };

//             if level_bounds.contains(player_transform.translation().xy())
//                 && *level_iid != current_map_level.level_iid
//             {
//                 info!("Player change level to {level_iid}");
//                 current_map_level.level_iid = level_iid.clone();
//                 let config = configs.iter().find_map(|(entity, config)| {
//                     parents
//                         .iter_ancestors(entity)
//                         .find(|p| *p == level_entity)
//                         .map(|_| config)
//                 });

//                 if let Some(config) = config {
//                     current_map_level.name = config.name.clone();
//                     current_map_level.monster_level = config.monster_level;
//                 } else {
//                     error!("Can't find MapLevelConfig for level {level_iid:?}");
//                 }
//             }
//         }
//     }
// }

fn world_map_picking_backend(
    pointers: Query<(&PointerId, &PointerLocation)>,
    camera: Single<(Entity, &Camera, &GlobalTransform), With<MainCamera>>,
    worlds_maps: Query<Entity, With<WorldMap>>,
    world_map: Res<ProceduralWorldMap>,
    mut output: EventWriter<PointerHits>,
) {
    let Ok(world_map) = worlds_maps.get_single() else {
        return;
    };
    let (camera_entity, camera, camera_transform) = *camera;
    for (pointer_id, pointer_location) in &pointers {
        let Some(pointer_world_pos) = pointer_location.world_position(camera, camera_transform)
        else {
            continue;
        };

        // let ldtk_project = ldtk_project_assets
        //     .get(ldtk_projects.single())
        //     .expect("ldtk project should be loaded before player is spawned");
        // let in_world_map = levels.iter().any(|(liid, lvl_transform)| {
        //     let level = ldtk_project
        //         .get_raw_level_by_iid(liid.get())
        //         .expect("level should exist in only project");

        //     let level_bounds = Rect {
        //         min: lvl_transform.translation().xy(),
        //         max: lvl_transform.translation().xy()
        //             + vec2(level.px_wid as f32, level.px_hei as f32),
        //     };

        //     level_bounds.contains(pointer_world_pos)
        // });
        let in_world_map = true;
        if in_world_map {
            let depth = MAP_DEPTH;
            let position = Some(pointer_world_pos.extend(0.));
            let picks = vec![(
                world_map,
                HitData::new(camera_entity, depth, position, Some(Vec3::Z)),
            )];
            let order = camera.order as f32;
            output.send(PointerHits::new(*pointer_id, picks, order));
        }
    }
}
