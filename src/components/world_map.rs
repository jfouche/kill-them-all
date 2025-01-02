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
