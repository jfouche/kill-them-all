mod characteristics_panel;
mod inventory_panel;
pub mod level_up_menu;
pub mod pause_menu;
pub mod player_died_menu;
pub mod round_end_menu;

use super::{pause, unpause, InGameState};
use crate::components::EquipmentAssets;
use bevy::{app::PluginGroupBuilder, prelude::*};
use characteristics_panel::characteristics_panel;
use inventory_panel::inventory_panel;

pub struct InGameMenuPluginsGroup;

impl PluginGroup for InGameMenuPluginsGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(pause_menu::PausePlugin)
            .add(level_up_menu::LevelUpMenuPlugin)
            .add(player_died_menu::PlayerDiedMenuPlugin)
            .add(round_end_menu::RoundEndMenuPlugin)
            .add(inventory_panel::inventory_panel_plugin)
            .add(characteristics_panel::CharacteristicsPanelPlugin)
            .add(menu_plugin)
    }
}

fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGameState::Pause), pause)
        .add_systems(OnExit(InGameState::Pause), unpause)
        .add_systems(OnEnter(InGameState::LevelUp), pause)
        .add_systems(OnExit(InGameState::LevelUp), unpause)
        .add_systems(OnEnter(InGameState::RoundEnd), pause)
        .add_systems(OnExit(InGameState::RoundEnd), unpause)
        .add_systems(Startup, load_assets);
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let assets = EquipmentAssets::load(&asset_server, texture_atlases);
    commands.insert_resource(assets);
}
