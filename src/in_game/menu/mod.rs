mod dnd;
mod item_location;
mod menu_level_up;
mod menu_pause;
mod menu_player_died;
mod panel_equipments;
mod panel_skills;
pub mod popup_info;
mod window_inventory;
mod window_statistics;

pub use plugin::InGameMenuPlugin;

mod plugin {
    use super::{
        dnd::DndPlugin, item_location::ItemLocationPlugin, menu_level_up::LevelUpMenuPlugin,
        menu_pause::PausePlugin, menu_player_died::PlayerDiedMenuPlugin,
        panel_equipments::EquipmentPanelPlugin, panel_skills::SkillsPanelPlugin,
        popup_info::PopupInfoPlugin, window_inventory::InventoryPanelPlugin,
        window_statistics::StatsWindowPlugin,
    };
    use crate::in_game::{pause, unpause};
    use crate::{components::item::ItemAssets, schedule::InGameState};
    use bevy::prelude::{App, OnEnter, OnExit, Plugin};

    pub struct InGameMenuPlugin;

    impl Plugin for InGameMenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((
                ItemLocationPlugin,
                PausePlugin,
                LevelUpMenuPlugin,
                PlayerDiedMenuPlugin,
                InventoryPanelPlugin,
                SkillsPanelPlugin,
                StatsWindowPlugin,
                EquipmentPanelPlugin,
                DndPlugin,
                PopupInfoPlugin,
            ))
            .init_resource::<ItemAssets>()
            .add_systems(OnEnter(InGameState::Pause), pause)
            .add_systems(OnExit(InGameState::Pause), unpause)
            .add_systems(OnEnter(InGameState::LevelUp), pause)
            .add_systems(OnExit(InGameState::LevelUp), unpause);
        }
    }
}
