use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::{rngs::ThreadRng, Rng};

#[derive(Resource)]
pub struct WorldMapAssets {
    pub ldtk_project: LdtkProjectHandle,
}

impl FromWorld for WorldMapAssets {
    fn from_world(world: &mut World) -> Self {
        WorldMapAssets {
            ldtk_project: world.load_asset("kill-them-all.ldtk").into(),
        }
    }
}

/// The world map
#[derive(Component, Copy, Clone)]
#[require(Name(|| Name::new("WorldMap")))]
pub struct WorldMap;

/// The player initial position tag
#[derive(Component, Default)]
pub struct PlayerInitialPosition;

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerInitialPositionLdtkBundle {
    tag: PlayerInitialPosition,
}

/// Monsters initial positions tag
#[derive(Component, Default)]
pub struct MonsterInitialPosition;

#[derive(Bundle, Default, LdtkEntity)]
pub struct MonsterInitialPositionLdtkBundle {
    tag: MonsterInitialPosition,
    #[from_entity_instance]
    count: MonsterCount,
}

#[derive(Component, Deref)]
pub struct MonsterCount(pub u16);

const DEFAULT_MONSTER_COUNT: u16 = 1;

impl Default for MonsterCount {
    fn default() -> Self {
        MonsterCount(DEFAULT_MONSTER_COUNT)
    }
}

impl From<&EntityInstance> for MonsterCount {
    fn from(value: &EntityInstance) -> Self {
        let count = value
            .get_int_field("count")
            .map(|v| u16::try_from(*v).unwrap_or(DEFAULT_MONSTER_COUNT))
            .unwrap_or(DEFAULT_MONSTER_COUNT);
        MonsterCount(count)
    }
}

/// Map colliders
#[derive(Bundle, Default, LdtkIntCell)]
pub struct ColliderLdtkBundle {
    collider: ColliderTile,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct WaterLdtkBundle {
    collider: WaterTile,
}

#[derive(Component, Default)]
#[require(
    Name(|| Name::new("WaterTile")),
    ColliderTile
)]
pub struct WaterTile;

impl WaterTile {
    pub const ID: i32 = 4;
}
#[derive(Component, Default)]
#[require(
    Name(|| Name::new("ColliderTile"))
)]
pub struct ColliderTile;

impl ColliderTile {
    pub const ID: i32 = 3;
}

#[derive(Component)]
#[require(
    Name(|| Name::new("Map Collider")),
    Transform,
    Collider,
    RigidBody(|| RigidBody::Fixed),
    Friction(|| Friction::new(1.0)),

)]
pub struct MapCollider;

/// Map level configuration
#[derive(Component, Reflect)]
pub struct MapLevelConfig {
    pub name: String,
    pub monster_level: u16,
}

#[derive(Bundle, LdtkEntity)]
pub struct LevelConfigLdtkBundle {
    #[from_entity_instance]
    config: MapLevelConfig,
}

impl From<&EntityInstance> for MapLevelConfig {
    fn from(value: &EntityInstance) -> Self {
        let name = value
            .get_string_field("name")
            .cloned()
            .expect("[name] should be defined for each LDtk level.");
        let monster_level = value
            .get_int_field("monster_level")
            .map(|v| u16::try_from(*v).unwrap_or(0))
            .expect("[monster_level] should be defined for each LDtk level.");
        MapLevelConfig {
            name,
            monster_level,
        }
    }
}

/// A resource to store the current map level informations
#[derive(Resource, Default)]
pub struct CurrentMapLevel {
    pub level_iid: LevelIid,
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

#[derive(Resource)]
pub struct ProceduralWorldMap {
    config: WorldMapConfig,
    tiles: Vec<WorldMapTile>,
    ground_map: HashSet<(usize, usize)>,
}

impl ProceduralWorldMap {
    pub fn generate(config: WorldMapConfig, rng: &mut ThreadRng) -> Self {
        let perlin = Perlin::new(rng.random());
        let tiles = Vec::new();
        let mut ground_map = HashSet::new();
        for x in 0..config.width {
            for y in 0..config.height {
                // value in [-1 .. 1]
                let noise_val =
                    perlin.get([x as f64 / config.noise_scale, y as f64 / config.noise_scale]);

                if noise_val > -0.4 {
                    ground_map.insert((x, y));
                }
            }
        }

        Self {
            config,
            tiles,
            ground_map,
        }
    }
}

impl std::fmt::Debug for ProceduralWorldMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.config.height {
            for x in 0..self.config.width {
                let c = match self.ground_map.get(&(x, y)) {
                    None => ' ',
                    Some(_) => 'X',
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct WorldMapTile {
    pos: (i32, i32),
    sprite: usize,
    // z_index: i32,
}

pub struct WorldMapConfig {
    width: usize,
    height: usize,
    noise_scale: f64,
}

impl Default for WorldMapConfig {
    fn default() -> Self {
        WorldMapConfig {
            width: 130,
            height: 15,
            noise_scale: 13.5,
        }
    }
}
