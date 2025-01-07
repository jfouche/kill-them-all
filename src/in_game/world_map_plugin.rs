use super::GameRunningSet;
use crate::{components::*, schedule::GameState};
use bevy::{math::vec2, prelude::*, utils::HashMap};
use bevy_ecs_ldtk::{prelude::*, utils::grid_coords_to_translation};
use bevy_rapier2d::prelude::*;
use std::{collections::HashSet, f32::consts::PI};

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .init_resource::<WorldMapAssets>()
            .insert_resource(LevelSelection::index(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..default()
            })
            .register_ldtk_int_cell::<WaterLdtkBundle>(WaterTile::ID)
            .register_ldtk_int_cell::<ColliderLdtkBundle>(ColliderTile::ID)
            .register_ldtk_entity::<PlayerInitialPositionLdtkBundle>("PlayerInitialPosition")
            .register_ldtk_entity::<MonsterInitialPositionLdtkBundle>("MonsterInitialPosition")
            .add_systems(OnEnter(GameState::InGame), spawn_worldmap)
            .add_systems(OnExit(GameState::InGame), despawn_all::<WorldMap>)
            .add_systems(
                Update,
                (
                    spawn_colliders,
                    level_selection_follow_player,
                    init_player_position,
                    init_monsters_position,
                )
                    .in_set(GameRunningSet::EntityUpdate),
            );
    }
}

fn spawn_worldmap(mut commands: Commands, assets: Res<WorldMapAssets>) {
    commands.insert_resource(LevelSelection::index(0));
    commands.spawn((
        WorldMap,
        LdtkWorldBundle {
            ldtk_handle: assets.ldtk_project.clone(),
            ..Default::default()
        },
    ));
}

/// Spawns colliders of a level
///
/// You could just insert a ColliderBundle into the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// The algorithm used here is a nice compromise between simplicity, speed,
/// and a small number of rectangle colliders.
/// In basic terms, it will:
/// 1. consider where the walls are
/// 2. combine wall tiles into flat "plates" in each individual row
/// 3. combine the plates into rectangles across multiple rows wherever possible
/// 4. spawn colliders for each rectangle
fn spawn_colliders(
    mut commands: Commands,
    added_colliders: Query<(&GridCoords, &Parent), Added<ColliderTile>>,
    parents: Query<&Parent, Without<ColliderTile>>,
    levels: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_collider_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    added_colliders.iter().for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parents.get(**parent) {
            level_to_collider_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });

    if !added_colliders.is_empty() {
        levels.iter().for_each(|(level_entity, level_iid)| {
            if let Some(level_colliders) = level_to_collider_locations.get(&level_entity) {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level.layer_instances()[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                    for x in 0..width + 1 {
                        match (plate_start, level_colliders.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut collider_rects: Vec<Rect> = Vec::new();

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                collider_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(Rect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for collider_rect in collider_rects {
                        level.spawn((
                            Name::new("Map Collider"),
                            Collider::cuboid(
                                (collider_rect.right as f32 - collider_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (collider_rect.top as f32 - collider_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                            ),
                            RigidBody::Fixed,
                            Friction::new(1.0),
                            Transform::from_xyz(
                                (collider_rect.left + collider_rect.right + 1) as f32
                                    * grid_size as f32
                                    / 2.,
                                (collider_rect.bottom + collider_rect.top + 1) as f32
                                    * grid_size as f32
                                    / 2.,
                                0.,
                            ),
                        ));
                    }
                });
            }
        });
    }
}

fn init_player_position(
    mut commands: Commands,
    mut players: Query<&mut Transform, With<Player>>,
    initial_positions: Query<(Entity, &GridCoords), With<PlayerInitialPosition>>,
) {
    if let Ok(mut player_transform) = players.get_single_mut() {
        for (entity, coord) in &initial_positions {
            player_transform.translation =
                grid_coords_to_translation(*coord, IVec2::splat(16)).extend(4.);
            info!(
                "init_player_position({})",
                player_transform.translation.xy()
            );
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn init_monsters_position(
    mut commands: Commands,
    monsters: Query<(Entity, &GridCoords, &MonsterCount), With<MonsterInitialPosition>>,
    assets: Res<AllMonsterAssets>,
) {
    let mut rng = rand::thread_rng();
    for (entity, coord, count) in &monsters {
        let pos = grid_coords_to_translation(*coord, IVec2::splat(16));
        for i in 0..**count {
            let angle = 2. * PI * f32::from(i) / f32::from(**count);
            let dist = 20.;
            let translation = pos + dist * vec2(angle.cos(), angle.sin());
            let translation = translation.extend(4.);

            let params = MonsterSpawnParams::generate(1, &mut rng);
            let scale = params.scale();

            let monster_components = (
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
        commands.entity(entity).despawn_recursive();
    }
}

fn level_selection_follow_player(
    players: Query<&GlobalTransform, With<Player>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    if let Ok(player_transform) = players.get_single() {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("ldtk project should be loaded before player is spawned");

        for (level_iid, level_transform) in levels.iter() {
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("level should exist in only project");

            let level_bounds = Rect {
                min: Vec2::new(
                    level_transform.translation().x,
                    level_transform.translation().y,
                ),
                max: Vec2::new(
                    level_transform.translation().x + level.px_wid as f32,
                    level_transform.translation().y + level.px_hei as f32,
                ),
            };

            if level_bounds.contains(player_transform.translation().truncate()) {
                if let LevelSelection::Iid(ref iid) = *level_selection {
                    if level_iid != iid {
                        info!("Player change level to {level_iid}");
                    }
                }
                *level_selection = LevelSelection::Iid(level_iid.clone());
            }
        }
    }
}
