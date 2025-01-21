mod menu_level_up;
mod menu_pause;
mod menu_player_died;
mod panel_equipments;
mod panel_skills;
mod popup_info;
mod popup_select_equipment;
mod window_inventory;
mod window_statistics;

use super::{pause, unpause};
use crate::{components::equipment::EquipmentAssets, schedule::InGameState};
use bevy::prelude::*;

pub struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            menu_pause::PausePlugin,
            menu_level_up::LevelUpMenuPlugin,
            menu_player_died::PlayerDiedMenuPlugin,
            panel_equipments::InventoryPanelPlugin,
            panel_skills::SkillsPanelPlugin,
            window_statistics::StatsWindowPlugin,
            window_inventory::InventoryPanelPlugin,
        ))
        .init_resource::<EquipmentAssets>()
        .add_systems(OnEnter(InGameState::Pause), pause)
        .add_systems(OnExit(InGameState::Pause), unpause)
        .add_systems(OnEnter(InGameState::LevelUp), pause)
        .add_systems(OnExit(InGameState::LevelUp), unpause);
    }
}
