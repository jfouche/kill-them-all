mod menu_level_up;
mod menu_pause;
mod menu_player_died;
mod panel_equipments;
mod panel_skills;
mod popup_info;
mod popup_select_equipment;
mod window_inventory;
mod window_statistics;

use super::{pause, unpause, InGameState};
use crate::components::EquipmentAssets;
use bevy::{app::PluginGroupBuilder, prelude::*};
use panel_equipments::EquipmentsPanel;
use popup_info::ShowPopupOnMouseOver;
use popup_select_equipment::ShowEquipmentActionsOnMouseOver;

pub struct InGameMenuPluginsGroup;

impl PluginGroup for InGameMenuPluginsGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(menu_pause::PausePlugin)
            .add(menu_level_up::LevelUpMenuPlugin)
            .add(menu_player_died::PlayerDiedMenuPlugin)
            .add(panel_equipments::InventoryPanelPlugin)
            .add(panel_skills::SkillsPanelPlugin)
            .add(window_statistics::StatsWindowPlugin)
            .add(window_inventory::InventoryPanelPlugin)
            .add(menu_plugin)
    }
}

fn menu_plugin(app: &mut App) {
    app.init_resource::<EquipmentAssets>()
        .add_systems(OnEnter(InGameState::Pause), pause)
        .add_systems(OnExit(InGameState::Pause), unpause)
        .add_systems(OnEnter(InGameState::LevelUp), pause)
        .add_systems(OnExit(InGameState::LevelUp), unpause);
}
