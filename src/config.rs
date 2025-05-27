use bevy::{
    asset::{io::file::FileAssetReader, ron},
    prelude::*,
};
use serde::Deserialize;

#[derive(Resource, Debug, Asset, TypePath, Deserialize)]
pub struct GameConfig {
    pub monster_spawn_delay: u64,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            monster_spawn_delay: 15,
        }
    }
}

#[derive(Event)]
pub struct ConfigLoaded;

pub struct GameConfigPlugin;

impl Plugin for GameConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameConfig>()
            .add_systems(PreStartup, load_config);
    }
}

fn load_config(mut commands: Commands) -> Result {
    let mut path = FileAssetReader::get_base_path();
    path.push("assets");
    path.push("config.ron");
    let config_string = std::fs::read_to_string(path.as_path())?;
    let config: GameConfig = ron::from_str(&config_string)?;
    info!("load_config: {config:?}");
    commands.insert_resource(config);
    commands.trigger(ConfigLoaded);
    Ok(())
}
