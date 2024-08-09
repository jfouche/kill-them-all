use crate::schedule::GameState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(Startup, load_assets)
            .add_systems(OnEnter(GameState::InGame), spawn_worldmap);
    }
}

const WORLD_WIDTH: f32 = 35.0;
const WORLD_HEIGH: f32 = 25.0;

const BORDER: f32 = 1.0;

#[derive(Resource)]
pub struct WorldMapAssets {
    texture: Handle<Image>,
}

#[derive(Bundle)]
struct WorldBundle {
    sprite: SpriteBundle,
}

impl WorldBundle {
    fn default() -> Self {
        WorldBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(WORLD_WIDTH, WORLD_HEIGH)),
                    color: Color::srgb(0.6, 0.6, 0.6),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
struct Border {
    sprite: SpriteBundle,
    collider: Collider,
}

impl Border {
    fn top() -> Self {
        Border {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(WORLD_WIDTH, BORDER)),
                    color: Color::NONE,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., WORLD_HEIGH / 2. + BORDER / 2., 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(WORLD_WIDTH / 2., BORDER / 2.),
        }
    }

    fn right() -> Self {
        Border {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BORDER, WORLD_HEIGH)),
                    color: Color::NONE,
                    ..Default::default()
                },
                transform: Transform::from_xyz(WORLD_WIDTH / 2. + BORDER / 2., 0.0, 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(BORDER / 2., WORLD_HEIGH / 2.),
        }
    }

    fn bottom() -> Self {
        Border {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(WORLD_WIDTH, BORDER)),
                    color: Color::NONE,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., -WORLD_HEIGH / 2. - BORDER / 2., 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(WORLD_WIDTH / 2., BORDER / 2.),
        }
    }

    fn left() -> Self {
        Border {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(WORLD_WIDTH, BORDER)),
                    color: Color::NONE,
                    ..Default::default()
                },
                transform: Transform::from_xyz(-WORLD_WIDTH / 2. - BORDER / 2., 0.0, 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(BORDER / 2., WORLD_HEIGH / 2.),
        }
    }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("background/TilesetFloor.png");
    let assets = WorldMapAssets { texture };
    commands.insert_resource(assets);
}

fn spawn_worldmap(mut commands: Commands, assets: Res<WorldMapAssets>) {
    // Create the tilemap which will be referenced by all tiles
    let map_size = TilemapSize { x: 32, y: 32 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands
        .spawn(Name::new("TileMap"))
        .with_children(|map| {
            // spawn tiles
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    let tile_pos = TilePos { x, y };
                    let tile_entity = map
                        .spawn((
                            Name::new("Tile"),
                            TileBundle {
                                position: tile_pos,
                                tilemap_id: TilemapId(map.parent_entity()),
                                ..Default::default()
                            },
                        ))
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        })
        .id();

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(assets.texture.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    // commands
    //     .spawn(WorldBundle::default())
    //     .insert(Name::new("World"))
    //     .with_children(|builder| {
    //         builder.spawn(Border::top());
    //         builder.spawn(Border::right());
    //         builder.spawn(Border::bottom());
    //         builder.spawn(Border::left());
    //     });
}
