use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

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

#[derive(Component, Copy, Clone)]
#[require(Name(|| Name::new("WorldMap")))]
pub struct WorldMap;

#[derive(Component, Default)]
pub struct PlayerInitialPosition;

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerInitialPositionLdtkBundle {
    tag: PlayerInitialPosition,
    initial_pos: InitialPosition,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Component, Default)]
pub struct MonsterInitialPosition;

#[derive(Bundle, Default, LdtkEntity)]
pub struct MonsterInitialPositionLdtkBundle {
    tag: MonsterInitialPosition,
    initial_pos: InitialPosition,
    #[grid_coords]
    grid_coords: GridCoords,
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

#[derive(Component, Default)]
pub struct InitialPosition;

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
