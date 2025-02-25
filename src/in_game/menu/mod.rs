mod dnd;
mod item_location;
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

pub use plugin::InGameMenuPlugin;

mod plugin {
    use crate::in_game::{pause, unpause};
    use bevy::prelude::{App, OnEnter, OnExit, Plugin};

    pub struct InGameMenuPlugin;

    impl Plugin for InGameMenuPlugin {
        fn build(&self, app: &mut App) {
            use super::{
                dnd::DndPlugin, item_location::ItemImagePlugin, menu_level_up::LevelUpMenuPlugin,
                menu_pause::PausePlugin, menu_player_died::PlayerDiedMenuPlugin,
                panel_equipments::InventoryPanelPlugin, panel_skills::SkillsPanelPlugin,
                window_statistics::StatsWindowPlugin,
            };
            use crate::{components::item::ItemAssets, schedule::InGameState};

            app.add_plugins((
                ItemImagePlugin,
                PausePlugin,
                LevelUpMenuPlugin,
                PlayerDiedMenuPlugin,
                InventoryPanelPlugin,
                SkillsPanelPlugin,
                StatsWindowPlugin,
                InventoryPanelPlugin,
                DndPlugin,
            ))
            .init_resource::<ItemAssets>()
            .add_systems(OnEnter(InGameState::Pause), pause)
            .add_systems(OnExit(InGameState::Pause), unpause)
            .add_systems(OnEnter(InGameState::LevelUp), pause)
            .add_systems(OnExit(InGameState::LevelUp), unpause);
        }
    }
}
