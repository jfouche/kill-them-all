mod dnd;
mod menu_level_up;
mod menu_pause;
mod menu_player_died;
mod panel_equipments;
mod panel_skills;
mod popup_info;
mod popup_select_equipment;
// mod popup_select_orb;
mod window_inventory;
mod window_statistics;

use bevy::prelude::{App, OnEnter, OnExit, Plugin};

pub struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin {
    fn build(&self, app: &mut App) {
        use super::{pause, unpause};
        use crate::{components::item::ItemAssets, schedule::InGameState};

        app.add_plugins((
            menu_pause::PausePlugin,
            menu_level_up::LevelUpMenuPlugin,
            menu_player_died::PlayerDiedMenuPlugin,
            panel_equipments::InventoryPanelPlugin,
            panel_skills::SkillsPanelPlugin,
            popup_info::InfoPopupPlugin,
            window_statistics::StatsWindowPlugin,
            window_inventory::InventoryPanelPlugin,
            dnd::DndPlugin,
        ))
        .init_resource::<ItemAssets>()
        .add_systems(OnEnter(InGameState::Pause), pause)
        .add_systems(OnExit(InGameState::Pause), unpause)
        .add_systems(OnEnter(InGameState::LevelUp), pause)
        .add_systems(OnExit(InGameState::LevelUp), unpause);
    }
}
