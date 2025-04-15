use bevy::{
    prelude::*,
    utils::{hashbrown::HashSet, HashMap},
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

// /// The player initial position tag
// #[derive(Component, Default)]
// pub struct PlayerInitialPosition;

// #[derive(Bundle, Default, LdtkEntity)]
// pub struct PlayerInitialPositionLdtkBundle {
//     tag: PlayerInitialPosition,
// }

/// Monsters initial positions tag
#[derive(Component, Default)]
pub struct MonsterInitialPosition;

// #[derive(Bundle, Default, LdtkEntity)]
// pub struct MonsterInitialPositionLdtkBundle {
//     tag: MonsterInitialPosition,
//     #[from_entity_instance]
//     count: MonsterCount,
// }

#[derive(Component, Deref)]
pub struct MonsterCount(pub u16);

const DEFAULT_MONSTER_COUNT: u16 = 1;

impl Default for MonsterCount {
    fn default() -> Self {
        MonsterCount(DEFAULT_MONSTER_COUNT)
    }
}

// impl From<&EntityInstance> for MonsterCount {
//     fn from(value: &EntityInstance) -> Self {
//         let count = value
//             .get_int_field("count")
//             .map(|v| u16::try_from(*v).unwrap_or(DEFAULT_MONSTER_COUNT))
//             .unwrap_or(DEFAULT_MONSTER_COUNT);
//         MonsterCount(count)
//     }
// }

// /// Map colliders
// #[derive(Bundle, Default, LdtkIntCell)]
// pub struct ColliderLdtkBundle {
//     collider: ColliderTile,
// }

// #[derive(Bundle, Default, LdtkIntCell)]
// pub struct WaterLdtkBundle {
//     collider: WaterTile,
// }

// #[derive(Component, Default)]
// #[require(
//     Name(|| Name::new("WaterTile")),
//     ColliderTile
// )]
// pub struct WaterTile;

// impl WaterTile {
//     pub const ID: i32 = 4;
// }
// #[derive(Component, Default)]
// #[require(
//     Name(|| Name::new("ColliderTile"))
// )]
// pub struct ColliderTile;

// impl ColliderTile {
//     pub const ID: i32 = 3;
// }

// #[derive(Component)]
// #[require(
//     Name(|| Name::new("Map Collider")),
//     Transform,
//     Collider,
//     RigidBody(|| RigidBody::Fixed),
//     Friction(|| Friction::new(1.0)),

// )]
// pub struct MapCollider;

/// Map level configuration
#[derive(Component, Reflect)]
pub struct MapLevelConfig {
    pub name: String,
    pub monster_level: u16,
}

// #[derive(Bundle, LdtkEntity)]
// pub struct LevelConfigLdtkBundle {
//     #[from_entity_instance]
//     config: MapLevelConfig,
// }

// impl From<&EntityInstance> for MapLevelConfig {
//     fn from(value: &EntityInstance) -> Self {
//         let name = value
//             .get_string_field("name")
//             .cloned()
//             .expect("[name] should be defined for each LDtk level.");
//         let monster_level = value
//             .get_int_field("monster_level")
//             .map(|v| u16::try_from(*v).unwrap_or(0))
//             .expect("[monster_level] should be defined for each LDtk level.");
//         MapLevelConfig {
//             name,
//             monster_level,
//         }
//     }
// }

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
    ground_map: HashMap<TilePos, TileTextureIndex>,
    spawned_chunks: HashSet<IVec2>,
}

impl ProceduralWorldMap {
    pub fn new(config: WorldMapConfig, rng: &mut ThreadRng) -> Self {
        ProceduralWorldMap {
            config,
            perlin: Perlin::new(rng.random()),
            ground_map: HashMap::new(),
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

    pub fn spawn_chunk(
        &mut self,
        commands: &mut Commands,
        assets: &WorldMapAssets,
        chunk_pos: IVec2,
    ) -> Entity {
        error!("spawn_chunk({chunk_pos})");
        self.spawned_chunks.insert(chunk_pos);
        let tilemap_entity = commands
            .spawn(Name::new(format!("WorldMapChunk {chunk_pos}")))
            .id();
        let translation = Vec3::new(
            chunk_pos.x as f32 * self.config.chunk_size as f32 * self.config.tile_size as f32,
            chunk_pos.y as f32 * self.config.chunk_size as f32 * self.config.tile_size as f32,
            0.0,
        );

        let mut tile_storage = TileStorage::empty(UVec2::splat(self.config.chunk_size).into());
        // Spawn the elements of the tilemap.
        for x in 0..self.config.chunk_size {
            for y in 0..self.config.chunk_size {
                let tile_pos = TilePos { x, y };

                let fx = translation.x as f64 + x as f64;
                let fy = translation.y as f64 + y as f64;
                error!("Perlin.get([{fx}, {fy}]");
                let noise_val = self
                    .perlin
                    .get([fx / self.config.noise_scale, fy / self.config.noise_scale]);
                // noise_val is in range [-1 .. 1]
                let kind = if noise_val < -0.4 {
                    TileKind::Water
                } else if noise_val < 0.5 {
                    TileKind::Mud
                } else {
                    TileKind::Grass
                };
                let i = match kind {
                    TileKind::Water => 34,
                    TileKind::Mud => 23,
                    TileKind::Grass => 264,
                };

                let tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(i),
                        ..Default::default()
                    })
                    .id();
                commands.entity(tilemap_entity).add_child(tile_entity);
                tile_storage.set(&tile_pos, tile_entity);
            }
        }

        let texture_handle: Handle<Image> = assets.sprites.clone();
        let tile_size = TilemapTileSize {
            x: self.config.tile_size as f32,
            y: self.config.tile_size as f32,
        };
        let chunk_size = UVec2::splat(self.config.chunk_size);
        commands.entity(tilemap_entity).insert(TilemapBundle {
            grid_size: tile_size.into(),
            size: chunk_size.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size: tile_size,
            transform: Transform::from_translation(translation),
            render_settings: TilemapRenderSettings {
                render_chunk_size: chunk_size * 2,
                ..Default::default()
            },
            ..Default::default()
        });
        tilemap_entity
    }

    // pub fn generate(config: WorldMapConfig, rng: &mut ThreadRng) -> Self {
    //     let perlin = Perlin::new(rng.random());
    //     let mut ground_map = HashMap::new();
    //     for x in 0..config.width {
    //         for y in 0..config.height {
    //             let noise_val =
    //                 perlin.get([x as f64 / config.noise_scale, y as f64 / config.noise_scale]);
    //             // noise_val is in range [-1 .. 1]
    //             let kind = if noise_val < -0.4 {
    //                 TileKind::Water
    //             } else if noise_val < 0.5 {
    //                 TileKind::Mud
    //             } else {
    //                 TileKind::Grass
    //             };
    //             ground_map.insert(
    //                 TilePos {
    //                     x: x as u32,
    //                     y: y as u32,
    //                 },
    //                 kind,
    //             );
    //         }
    //     }

    //     let ground_map = ground_map
    //         .iter()
    //         .map(|(&p, k)| {
    //             let i = match k {
    //                 TileKind::Water => 34,
    //                 TileKind::Mud => 23,
    //                 TileKind::Grass => 264,
    //             };
    //             (p, TileTextureIndex(i))
    //         })
    //         .collect::<HashMap<_, _>>();

    //     Self {
    //         config,
    //         perlin,
    //         ground_map,
    //     }
    // }

    pub fn pos_to_world(&self, x: i32, y: i32) -> Vec2 {
        let pos = IVec2 { x, y };
        let tile_size: IVec2 = IVec2::splat(self.config.tile_size as i32);
        (pos * tile_size).as_vec2()
    }

    pub fn world_to_pos(&self, translation: Vec2) -> IVec2 {
        (translation / Vec2::splat(self.config.tile_size as f32)).as_ivec2()
    }

    // pub fn tilemap_chunk(&self, camera_pos: Vec2) -> Vec<TileBundle> {
    //     let tiles = Vec::with_capacity((self.config.chunk_size * self.config.chunk_size) as usize);
    //     let chunk_pos = self.camera_pos_to_chunk_pos(camera_pos);
    //     dbg!(chunk_pos);

    //     tiles
    // }

    // pub fn spawn(&self, commands: &mut Commands, assets: &WorldMapAssets) {
    //     let map_size = TilemapSize {
    //         x: self.config.width as u32,
    //         y: self.config.height as u32,
    //     };
    //     let mut tile_storage = TileStorage::empty(map_size);
    //     let tilemap_entity = commands.spawn(WorldMap).id();

    //     for (&position, &texture_index) in &self.ground_map {
    //         let tile_entity = commands
    //             .spawn(TileBundle {
    //                 position,
    //                 tilemap_id: TilemapId(tilemap_entity),
    //                 texture_index,
    //                 ..Default::default()
    //             })
    //             .id();
    //         tile_storage.set(&position, tile_entity);
    //         commands.entity(tilemap_entity).add_child(tile_entity);
    //     }
    //     let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    //     let grid_size = tile_size.into();
    //     let map_type = TilemapType::default();

    //     commands.entity(tilemap_entity).insert(TilemapBundle {
    //         grid_size,
    //         map_type,
    //         size: map_size,
    //         storage: tile_storage,
    //         texture: TilemapTexture::Single(assets.sprites.clone()),
    //         tile_size,
    //         ..Default::default()
    //     });

    // // spawn exterior colliders
    // commands.entity(tilemap_entity).with_children(|parent| {
    //     const HW: f32 = 2.0;
    //     let left = self.pos_to_world(-1, 0).x;
    //     error!("left={left} for (-1, 0)");
    //     let right = self.pos_to_world(self.config.width as i32, 0).x;
    //     error!("right={right} for ({}), 0)", self.config.width);
    //     let half_x = (right - left) / 2.;
    //     error!("half_x={half_x}; left + half_x={}", left + half_x);
    //     parent.spawn((
    //         Collider::cuboid(half_x, HW),
    //         Transform::from_translation(vec3(left + half_x, HW, 0.)),
    //     ));
    // });
    // }
}

// #[derive(Component)]
// pub struct WorldMapChunk;

#[derive(Clone, Copy, Debug)]
enum TileKind {
    Water,
    Mud,
    Grass,
}

pub struct WorldMapConfig {
    noise_scale: f64,
    chunk_size: u32,
    tile_size: u32,
}

impl Default for WorldMapConfig {
    fn default() -> Self {
        WorldMapConfig {
            noise_scale: 13.5,
            chunk_size: 10,
            tile_size: 16,
        }
    }
}
