use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct WorldMapAssets {
    pub texture: Handle<Image>,
}

#[derive(Component, Copy, Clone)]
pub struct WorldMap;

#[derive(Bundle)]
pub struct WorldMapBundle {
    tag: WorldMap,
    name: Name,
    tilemap: TilemapBundle,
}

impl WorldMapBundle {
    pub fn new(builder: WorldMapBuilder, assets: &WorldMapAssets) -> Self {
        let grid_size = builder.tile_size.into();
        let transform =
            get_tilemap_center_transform(&builder.map_size, &grid_size, &TilemapType::Square, 0.0);
        WorldMapBundle {
            tag: WorldMap,
            name: Name::new("WorldMap"),
            tilemap: TilemapBundle {
                grid_size,
                map_type: TilemapType::Square,
                size: builder.map_size,
                storage: builder.tile_storage,
                texture: TilemapTexture::Single(assets.texture.clone()),
                tile_size: builder.tile_size,
                transform,
                ..Default::default()
            },
        }
    }
}

#[derive(Component)]
pub struct WorldMapCollider;

#[derive(Bundle)]
pub struct WorldMapColliderBundle {
    tag: WorldMapCollider,
    name: Name,
    collider: Collider,
    transform: Transform,
}

impl Default for WorldMapColliderBundle {
    fn default() -> Self {
        WorldMapColliderBundle {
            tag: WorldMapCollider,
            name: Name::new("WorldMapCollider"),
            collider: Collider::default(),
            transform: Transform::default(),
        }
    }
}

pub struct WorldMapBuilder {
    map_size: TilemapSize,
    tile_size: TilemapTileSize,
    tile_storage: TileStorage,
    map_type: u32,
}

impl WorldMapBuilder {
    pub fn new(map_size: TilemapSize, tile_size: TilemapTileSize) -> Self {
        WorldMapBuilder {
            map_size,
            tile_size,
            tile_storage: TileStorage::empty(map_size),
            map_type: rand::thread_rng().gen_range(0..8),
        }
    }

    fn index(&self, pos: &TilePos) -> u32 {
        let top = self.map_size.y - 1;
        let bottom = 0;
        let left = 0;
        let right = self.map_size.x - 1;

        const TOP_LEFT_INDEX: u32 = 0;
        const TOP_RIGHT_INDEX: u32 = 2;
        const BOTTOM_LEFT_INDEX: u32 = 44;
        const BOTTOM_RIGHT_INDEX: u32 = 46;
        const TOP_INDEX: u32 = 1;
        const BOTTOM_INDEX: u32 = 45;
        const LEFT_INDEX: u32 = 22;
        const RIGHT_INDEX: u32 = 24;
        const FLOOR1_INDEX: u32 = 88;
        const FLOOR2_INDEX: u32 = 89;
        // const OBSTACLE_INDEX: u32 = 111;
        const DEFAULT_INDEX: u32 = 23;

        let idx = if pos.y == top && pos.x == left {
            TOP_LEFT_INDEX
        } else if pos.y == top && pos.x == right {
            TOP_RIGHT_INDEX
        } else if pos.y == bottom && pos.x == left {
            BOTTOM_LEFT_INDEX
        } else if pos.y == bottom && pos.x == right {
            BOTTOM_RIGHT_INDEX
        } else if pos.y == top {
            TOP_INDEX
        } else if pos.y == bottom {
            BOTTOM_INDEX
        } else if pos.x == left {
            LEFT_INDEX
        } else if pos.x == right {
            RIGHT_INDEX
        } else {
            match rand::thread_rng().gen_range(0..100) {
                0..=6 => FLOOR1_INDEX,
                7..=13 => FLOOR2_INDEX,
                // 14..=15 => OBSTACLE_INDEX,
                _ => DEFAULT_INDEX,
            }
        };
        // add an offset to match the map images in the atlas
        idx + 7 * 11 * 2 * (self.map_type / 2) + (self.map_type % 2) * 11
    }

    pub fn spawn_tile(&mut self, child_builder: &mut ChildBuilder, pos: &TilePos) {
        let tilemap_entity = child_builder.parent_entity();
        let tile_entity = child_builder
            .spawn((
                Name::new("Tile"),
                TileBundle {
                    position: *pos,
                    texture_index: TileTextureIndex(self.index(pos)),
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                },
            ))
            .id();
        self.tile_storage.set(pos, tile_entity);
    }

    pub fn spawn_colliders(&self, child_builder: &mut ChildBuilder) {
        let grid_size = self.tile_size.into();
        let map_type = TilemapType::Square;
        let Vec2 { x: left, y: bottom } =
            TilePos { x: 0, y: 0 }.center_in_world(&grid_size, &map_type);
        let Vec2 { x: right, y: top } = TilePos {
            x: self.map_size.x - 1,
            y: self.map_size.y - 1,
        }
        .center_in_world(&grid_size, &map_type);
        let x_center = (right - left) / 2.;
        let y_center = (top - bottom) / 2.;
        let half_w = x_center;
        let half_h = y_center;
        const COLLIDER_HALF_WIDTH: f32 = 1.0;

        // TOP
        child_builder.spawn(WorldMapColliderBundle {
            collider: Collider::cuboid(half_w, COLLIDER_HALF_WIDTH),
            transform: Transform::from_xyz(x_center, top, 0.),
            ..Default::default()
        });

        // BOTTOM
        child_builder.spawn(WorldMapColliderBundle {
            collider: Collider::cuboid(half_w, COLLIDER_HALF_WIDTH),
            transform: Transform::from_xyz(x_center, bottom, 0.),
            ..Default::default()
        });

        // LEFT
        child_builder.spawn(WorldMapColliderBundle {
            collider: Collider::cuboid(COLLIDER_HALF_WIDTH, half_h),
            transform: Transform::from_xyz(left, y_center, 0.),
            ..Default::default()
        });

        // RIGHT
        child_builder.spawn(WorldMapColliderBundle {
            collider: Collider::cuboid(COLLIDER_HALF_WIDTH, half_h),
            transform: Transform::from_xyz(right, y_center, 0.),
            ..Default::default()
        });
    }
}
