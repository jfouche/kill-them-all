use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
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
#[require(Name::new("WorldMap"), Transform, Visibility)]
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
pub const LAYER_MAP: f32 = 0.;

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

    pub fn chunk_pos(&self, translation: Vec2) -> IVec2 {
        let pos = translation.as_ivec2();
        let chunk_size: IVec2 = IVec2::splat(self.config.chunk_size as i32);
        let tile_size: IVec2 = IVec2::splat(self.config.tile_size as i32);
        let mut chunk_pos = pos / (chunk_size * tile_size);
        // Fix the calculus for negative values
        if translation.x < 0. {
            chunk_pos.x -= 1;
        }
        if translation.y < 0. {
            chunk_pos.y -= 1;
        }
        chunk_pos
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

    fn neighboors(&mut self, x: i32, y: i32) -> Neighbors {
        Neighbors([
            self.tile_kind(x - 1, y + 1),
            self.tile_kind(x + 0, y + 1),
            self.tile_kind(x + 1, y + 1),
            self.tile_kind(x - 1, y + 0),
            self.tile_kind(x + 0, y + 0),
            self.tile_kind(x + 1, y + 0),
            self.tile_kind(x - 1, y - 1),
            self.tile_kind(x + 0, y - 1),
            self.tile_kind(x + 1, y - 1),
        ])
    }

    fn tile_index(&mut self, x: i32, y: i32, rng: &mut ThreadRng) -> u32 {
        let n = self.neighboors(x, y);
        match n.c() {
            TileKind::Water => {
                let rules = GenericNeighborRules(&n);
                rules.index()
            }
            TileKind::Mud => {
                let rules = GenericNeighborRules(&n);
                rules.index()
            }
            TileKind::Grass => match rng.random_range(0..100) {
                0..5 => 244,
                5..10 => 265,
                10..15 => 266,
                15..20 => 267,
                20..35 => 245,
                _ => 264,
            },
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

        let mut rng = rand::rng();
        let mut tile_storage = TileStorage::empty(UVec2::splat(chunk_size).into());
        for x in 0..chunk_size {
            for y in 0..chunk_size {
                // (x, y) is in "chunk" coordinates, add offset to get the "world map" coordinates
                let index = self.tile_index(x as i32 + x_offset, y as i32 + y_offset, &mut rng);
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

        let f_tile_size = self.config.tile_size as f32;
        let texture_handle: Handle<Image> = assets.sprites.clone();
        let tile_size = TilemapTileSize {
            x: f_tile_size,
            y: f_tile_size,
        };

        let translation = Vec3::new(
            x_offset as f32 * f_tile_size,
            y_offset as f32 * f_tile_size,
            LAYER_MAP,
        );
        let chunk_size = UVec2::splat(chunk_size);
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
        // TODO : retain self.tiles_kind
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

struct Neighbors([TileKind; 9]);

impl Neighbors {
    #[inline]
    fn tl(&self) -> TileKind {
        self.0[0]
    }
    #[inline]
    fn t(&self) -> TileKind {
        self.0[1]
    }
    #[inline]
    fn tr(&self) -> TileKind {
        self.0[2]
    }
    #[inline]
    fn l(&self) -> TileKind {
        self.0[3]
    }
    #[inline]
    fn c(&self) -> TileKind {
        self.0[4]
    }
    #[inline]
    fn r(&self) -> TileKind {
        self.0[5]
    }
    #[inline]
    fn bl(&self) -> TileKind {
        self.0[6]
    }
    #[inline]
    fn b(&self) -> TileKind {
        self.0[7]
    }
    #[inline]
    fn br(&self) -> TileKind {
        self.0[8]
    }
}

/// O : match the center tile
///
/// X : different from the center tile
///
/// U : donc care about the tile
macro_rules! neighbors_match {
    ($sel: ident, $index:literal,
        $tl:ident $t:ident $tr:ident $l:ident $r:ident $bl:ident $b:ident $br:ident) => {
        (tile_match!($sel, tl, $tl)
            && tile_match!($sel, t, $t)
            && tile_match!($sel, tr, $tr)
            && tile_match!($sel, l, $l)
            && tile_match!($sel, r, $r)
            && tile_match!($sel, bl, $bl)
            && tile_match!($sel, b, $b)
            && tile_match!($sel, br, $br))
        .then(|| $index)
    };
    ($sel: ident, $index:literal FLIP_X $index_x:literal,
        $tl:ident $t:ident $tr:ident $l:ident $r:ident $bl:ident $b:ident $br:ident) => {
        neighbors_match!($sel, $index, $tl $t $tr $l $r $bl $b $br)
            .or(neighbors_match!($sel, $index_x, $tr $t $tl $r $l $br $b $bl))
    };
    ($sel: ident, $index:literal FLIP_Y $index_y:literal,
        $tl:ident $t:ident $tr:ident $l:ident $r:ident $bl:ident $b:ident $br:ident) => {
        neighbors_match!($sel, $index, $tl $t $tr $l $r $bl $b $br)
            .or(neighbors_match!($sel, $index_y, $bl $b $br $l $r $tl $t $tr))
    };
    ($sel: ident, $index:literal FLIP_XY $index_x:literal $index_y:literal $index_xy:literal,
        $tl:ident $t:ident $tr:ident $l:ident $r:ident $bl:ident $b:ident $br:ident) => {
        neighbors_match!($sel, $index, $tl $t $tr $l $r $bl $b $br)
            .or(neighbors_match!($sel, $index_x, $tr $t $tl $r $l $br $b $bl))
            .or(neighbors_match!($sel, $index_y, $bl $b $br $l $r $tl $t $tr))
            .or(neighbors_match!($sel, $index_xy, $br $b $bl $r $l $tr $t $tl))
    };
}

macro_rules! tile_match {
    ($sel: ident, $p:ident, O) => {
        $sel.$p() == $sel.c()
    };
    ($sel: ident, $p:ident, X) => {
        $sel.$p() != $sel.c()
    };
    ($sel: ident, $p:ident, U) => {
        true
    };
}

#[derive(Deref)]
struct GenericNeighborRules<'a>(&'a Neighbors);

impl<'a> GenericNeighborRules<'a> {
    fn index(&self) -> u32 {
        let offset = match self.c() {
            TileKind::Water => 11,
            TileKind::Mud => 154,
            _ => unreachable!("Only Water and Mud can have generic rules"),
        };
        let i = self
            .rule_5()
            .or(self.rule_7())
            .or(self.rule_8())
            .or(self.rule_9())
            .or(self.rule_10())
            .unwrap_or(23);
        offset + i
    }

    fn rule_5(&self) -> Option<u32> {
        neighbors_match!(self, 3 FLIP_Y 47,
            U X U
            X   X
            U U U
        )
    }

    fn rule_7(&self) -> Option<u32> {
        neighbors_match!(self, 0 FLIP_XY 2 44 46,
            U X U
            X   U
            U U U
        )
    }

    fn rule_8(&self) -> Option<u32> {
        neighbors_match!(self, 1 FLIP_Y 45,
            U X U
            U   U
            U U U
        )
    }

    fn rule_9(&self) -> Option<u32> {
        neighbors_match!(self, 22 FLIP_X 24,
            U U U
            X   U
            U U U
        )
    }

    fn rule_10(&self) -> Option<u32> {
        neighbors_match!(self, 50 FLIP_XY 49 28 27,
            X O U
            O   U
            U U U
        )
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
    pub noise_scale: f64,
    pub chunk_size: u32,
    pub tile_size: u32,
    pub despawn_distance: f32,
}

impl Default for WorldMapConfig {
    fn default() -> Self {
        WorldMapConfig {
            noise_scale: 13.5,
            chunk_size: 3,
            tile_size: 16,
            despawn_distance: 520.,
        }
    }
}
