pub mod level_up_menu;
pub mod pause_menu;
pub mod player_died_menu;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct InGameMenuPluginsGroup;

impl PluginGroup for InGameMenuPluginsGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(pause_menu::PausePlugin)
            .add(level_up_menu::LevelUpMenuPlugin)
            .add(player_died_menu::PlayerDiedPlugin)
    }
}
