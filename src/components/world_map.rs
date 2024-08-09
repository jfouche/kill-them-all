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

pub struct WorldMapBuilder {
    size: TilemapSize,
}

impl WorldMapBuilder {
    pub fn new(size: TilemapSize) -> Self {
        WorldMapBuilder { size }
    }

    pub fn generate_tile(&self, pos: TilePos, tilemap_entity: Entity) -> TileBundle {
        let mut rng = rand::thread_rng();

        let top = self.size.y - 1;
        let bottom = 0;
        let left = 0;
        let right = self.size.x - 1;

        let index = if pos.y == top && pos.x == left {
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
            match rng.gen_range(0..100) {
                0..7 => FLOOR1_INDEX,
                7..14 => FLOOR2_INDEX,
                _ => DEFAULT_INDEX,
            }
        };
        TileBundle {
            position: pos,
            texture_index: TileTextureIndex(index),
            tilemap_id: TilemapId(tilemap_entity),
            ..Default::default()
        }
    }
}

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
const DEFAULT_INDEX: u32 = 23;
