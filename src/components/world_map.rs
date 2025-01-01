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
pub struct WorldMap;

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerLdtkBundle {
    initial_pos: InitialPosition,
}

#[derive(Component, Default)]
pub struct InitialPosition;
