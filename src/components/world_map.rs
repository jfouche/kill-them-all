use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapRenderSettings, TilemapTexture, TilemapTileSize},
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle,
};
use noise::{NoiseFn, Perlin};
use rand::{rngs::ThreadRng, Rng};

#[derive(Resource)]
pub struct WorldMapAssets {
    pub sprites: Handle<Image>,
}

impl FromWorld for WorldMapAssets {
    fn from_world(world: &mut World) -> Self {
        WorldMapAssets {
            sprites: world.load_asset("kte-floor.png"),
        }
    }
}

/// The world map
#[derive(Component, Copy, Clone)]
#[require(
    Name(|| Name::new("WorldMap")),
    Transform,
    Visibility
)]
pub struct WorldMap;

/// Monsters initial positions tag
#[derive(Component, Default)]
pub struct MonsterInitialPosition;

#[derive(Component, Deref)]
pub struct MonsterCount(pub u16);

const DEFAULT_MONSTER_COUNT: u16 = 1;

impl Default for MonsterCount {
    fn default() -> Self {
        MonsterCount(DEFAULT_MONSTER_COUNT)
    }
}

/// Map level configuration
#[derive(Component, Reflect)]
pub struct MapLevelConfig {
    pub name: String,
    pub monster_level: u16,
}

/// A resource to store the current map level informations
#[derive(Resource, Default)]
pub struct CurrentMapLevel {
    pub name: String,
    pub monster_level: u16,
}

/// Event triggered when the map loading finished
#[derive(Event)]
pub struct WorldMapLoadingFinished {
    pub translation: Vec2,
}

/// Event triggered when the monsters can be spawn
#[derive(Event, Default)]
pub struct SpawnMonstersEvent {
    pub mlevel: u16,
    pub monsters: Vec<(Vec2, u16)>,
}

pub const LAYER_PLAYER: f32 = 10.;
pub const LAYER_MONSTER: f32 = 9.;
pub const LAYER_DAMAGER: f32 = 8.;
pub const LAYER_ITEM: f32 = 7.;

// ============================================================================
//
// PROCEDURAL WORLD MAP GENERATOR
//
// ============================================================================
#[derive(Resource)]
pub struct ProceduralWorldMap {
    config: WorldMapConfig,
    perlin: Perlin,
    tiles_kind: HashMap<(i32, i32), TileKind>,
    spawned_chunks: HashSet<IVec2>,
}

impl ProceduralWorldMap {
    pub fn new(config: WorldMapConfig, rng: &mut ThreadRng) -> Self {
        ProceduralWorldMap {
            config,
            perlin: Perlin::new(rng.random()),
            tiles_kind: HashMap::new(),
            spawned_chunks: HashSet::new(),
        }
    }

    pub fn camera_pos_to_chunk_pos(&self, camera_pos: Vec2) -> IVec2 {
        let camera_pos = camera_pos.as_ivec2();
        let chunk_size: IVec2 = IVec2::splat(self.config.chunk_size as i32);
        let tile_size: IVec2 = IVec2::splat(self.config.tile_size as i32);
        camera_pos / (chunk_size * tile_size)
    }

    pub fn is_spawned(&self, pos: IVec2) -> bool {
        self.spawned_chunks.contains(&pos)
    }

    fn tile_kind(&mut self, x: i32, y: i32) -> TileKind {
        match self.tiles_kind.get(&(x, y)) {
            Some(kind) => *kind,
            None => {
                let noise_val = self.perlin.get([
                    x as f64 / self.config.noise_scale,
                    y as f64 / self.config.noise_scale,
                ]);
                let kind = if noise_val < -0.4 {
                    TileKind::Water
                } else if noise_val < 0.5 {
                    TileKind::Mud
                } else {
                    TileKind::Grass
                };
                self.tiles_kind.insert((x, y), kind);
                kind
            }
        }
    }

    fn tile_index(&mut self, x: i32, y: i32) -> u32 {
        let neighbors = [
            [
                self.tile_kind(x - 1, y + 1),
                self.tile_kind(x + 0, y + 1),
                self.tile_kind(x + 1, y + 1),
            ],
            [
                self.tile_kind(x - 1, y + 0),
                self.tile_kind(x + 0, y + 0),
                self.tile_kind(x + 1, y + 0),
            ],
            [
                self.tile_kind(x - 1, y - 1),
                self.tile_kind(x + 0, y - 1),
                self.tile_kind(x + 1, y - 1),
            ],
        ];
        const M: TileKind = TileKind::Mud;
        const G: TileKind = TileKind::Grass;
        const W: TileKind = TileKind::Water;
        match neighbors {
            [[M, M, M], [G, M, M], [M, M, M]] => 176,
            [[_, _, _], [_, M, _], [_, _, _]] => 177,
            [[_, _, _], [_, G, _], [_, _, _]] => 275,
            [[_, _, _], [_, W, _], [_, _, _]] => 34,
        }
    }

    pub fn spawn_chunk(
        &mut self,
        commands: &mut Commands,
        assets: &WorldMapAssets,
        chunk_pos: IVec2,
    ) -> Entity {
        let chunk_size = self.config.chunk_size;
        let x_offset = chunk_pos.x * chunk_size as i32;
        let y_offset = chunk_pos.y * chunk_size as i32;

        let chunk_entity = commands
            .spawn((
                WorldMapChunk,
                Name::new(format!("WorldMapChunk {chunk_pos}")),
            ))
            .id();

        let mut tile_storage = TileStorage::empty(UVec2::splat(chunk_size).into());
        for x in 0..chunk_size {
            for y in 0..chunk_size {
                // (x, y) is in "chunk" ccordinates, add offset to get the "world" coord
                let index = self.tile_index(x as i32 + x_offset, y as i32 + y_offset);
                let tile_pos = TilePos { x, y };
                let tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(chunk_entity),
                        texture_index: TileTextureIndex(index),
                        ..Default::default()
                    })
                    .id();
                commands.entity(chunk_entity).add_child(tile_entity);
                tile_storage.set(&tile_pos, tile_entity);
            }
        }

        let texture_handle: Handle<Image> = assets.sprites.clone();
        let tile_size = TilemapTileSize {
            x: self.config.tile_size as f32,
            y: self.config.tile_size as f32,
        };
        let chunk_size = UVec2::splat(chunk_size);
        let translation = Vec3::new(
            x_offset as f32 * self.config.tile_size as f32,
            y_offset as f32 * self.config.tile_size as f32,
            0.0,
        );
        commands.entity(chunk_entity).insert(TilemapBundle {
            grid_size: tile_size.into(),
            size: chunk_size.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: Transform::from_translation(translation),
            render_settings: TilemapRenderSettings {
                render_chunk_size: chunk_size * 2,
                ..Default::default()
            },
            ..Default::default()
        });
        self.spawned_chunks.insert(chunk_pos);

        chunk_entity
    }

    pub fn remove_chunk_if_out_of_bound(&mut self, pos: Vec2, distance: f32) -> bool {
        if distance > self.config.despawn_distance {
            let ratio = (self.config.chunk_size * self.config.tile_size) as f32;
            let x = (pos.x / ratio).floor() as i32;
            let y = (pos.y / ratio).floor() as i32;
            self.spawned_chunks.remove(&IVec2::new(x, y));
            true
        } else {
            false
        }
    }

    pub fn pos_to_world(&self, x: i32, y: i32) -> Vec2 {
        let pos = IVec2 { x, y };
        let tile_size: IVec2 = IVec2::splat(self.config.tile_size as i32);
        (pos * tile_size).as_vec2()
    }

    pub fn world_to_pos(&self, translation: Vec2) -> IVec2 {
        (translation / Vec2::splat(self.config.tile_size as f32)).as_ivec2()
    }
}

#[derive(Component)]
pub struct WorldMapChunk;

#[derive(Clone, Copy, Debug, PartialEq)]
enum TileKind {
    Water,
    Mud,
    Grass,
}

pub struct WorldMapConfig {
    noise_scale: f64,
    chunk_size: u32,
    tile_size: u32,
    despawn_distance: f32,
}

impl Default for WorldMapConfig {
    fn default() -> Self {
        WorldMapConfig {
            noise_scale: 13.5,
            chunk_size: 4,
            tile_size: 16,
            despawn_distance: 320.,
        }
    }
}
