pub mod level_up_menu;
pub mod pause_menu;
pub mod player_died_menu;

use super::{pause, unpause, InGameState};
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct InGameMenuPluginsGroup;

impl PluginGroup for InGameMenuPluginsGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(pause_menu::PausePlugin)
            .add(level_up_menu::LevelUpMenuPlugin)
            .add(player_died_menu::PlayerDiedPlugin)
            .add(menu_plugin)
    }
}

fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGameState::Pause), pause)
        .add_systems(OnExit(InGameState::Pause), unpause)
        .add_systems(OnEnter(InGameState::LevelUp), pause)
        .add_systems(OnExit(InGameState::LevelUp), unpause);
}
