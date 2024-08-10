use crate::{components::*, schedule::GameState};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(Startup, load_assets)
            .add_systems(OnEnter(GameState::InGame), spawn_worldmap)
            .add_systems(OnExit(GameState::InGame), despawn_all::<WorldMap>);
    }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("background/TilesetFloor.png");
    let assets = WorldMapAssets { texture };
    commands.insert_resource(assets);
}

fn spawn_worldmap(mut commands: Commands, assets: Res<WorldMapAssets>) {
    let map_size = TilemapSize { x: 32, y: 32 };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let mut map_builder = WorldMapBuilder::new(map_size, tile_size);

    // Create the tilemap which will be referenced by all tiles
    commands
        .spawn((WorldMap, Name::new("WorldMap")))
        .with_children(|map| {
            // spawn tiles
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    let tile_pos = TilePos { x, y };
                    map_builder.spawn_tile(map, &tile_pos);
                }
            }
            // spawn colliders
            map_builder.spawn_colliders(map);
        })
        .insert(WorldMapBundle::new(map_builder, &assets));
}
