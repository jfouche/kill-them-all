mod menu_level_up;
mod menu_pause;
mod menu_player_died;
mod menu_round_end;
mod panel_characteristics;
mod panel_equipments;
mod panel_skills;
mod popup_info;

use super::{pause, unpause, InGameState};
use crate::components::EquipmentAssets;
use bevy::{app::PluginGroupBuilder, prelude::*};
use panel_characteristics::CharacteristicsPanel;
use panel_equipments::EquipmentsPanel;
use panel_skills::SkillsPanel;

pub struct InGameMenuPluginsGroup;

impl PluginGroup for InGameMenuPluginsGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(menu_pause::PausePlugin)
            .add(menu_level_up::LevelUpMenuPlugin)
            .add(menu_player_died::PlayerDiedMenuPlugin)
            .add(menu_round_end::RoundEndMenuPlugin)
            .add(panel_equipments::inventory_panel_plugin)
            .add(panel_skills::SkillsPanelPlugin)
            .add(panel_characteristics::CharacteristicsPanelPlugin)
            .add(menu_plugin)
    }
}

fn menu_plugin(app: &mut App) {
    app.init_resource::<EquipmentAssets>()
        .add_systems(OnEnter(InGameState::Pause), pause)
        .add_systems(OnExit(InGameState::Pause), unpause)
        .add_systems(OnEnter(InGameState::LevelUp), pause)
        .add_systems(OnExit(InGameState::LevelUp), unpause)
        .add_systems(OnEnter(InGameState::RoundEnd), pause)
        .add_systems(OnExit(InGameState::RoundEnd), unpause);
}
