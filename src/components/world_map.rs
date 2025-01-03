use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerInitialPositionLdtkBundle {
    initial_pos: InitialPosition,
    #[grid_coords]
    grid_coords: GridCoords,
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
