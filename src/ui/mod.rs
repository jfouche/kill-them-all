pub mod button;
pub mod item_location;
mod menu_level_up;
mod menu_pause;
mod menu_player_died;
mod panel_equipments;
mod panel_skills;
pub mod popup;
pub mod popup_info;
pub mod progressbar;
mod window_inventory;
mod window_statistics;

pub use plugin::{HSizer, UiPlugins, VSizer};

mod plugin {
    use super::*;
    use bevy::prelude::*;

    /// Horizontal sizer
    #[derive(Component, Default)]
    #[require(
    Node {
            flex_direction: FlexDirection::Row,
            ..Default::default()
        }
    )]
    pub struct HSizer;

    /// Vertical sizer
    #[derive(Component, Default)]
    #[require(
    Node {
        flex_direction: FlexDirection::Column,
        ..Default::default()
    }
)]
    pub struct VSizer;

    pub struct UiPlugins;

    impl Plugin for UiPlugins {
        fn build(&self, app: &mut App) {
            app.add_plugins((
                progressbar::ProgressBarPlugin,
                button::button_plugin,
                item_location::ItemLocationPlugin,
                menu_pause::PausePlugin,
                menu_level_up::LevelUpMenuPlugin,
                menu_player_died::PlayerDiedMenuPlugin,
                window_inventory::InventoryPanelPlugin,
                panel_skills::SkillsPanelPlugin,
                window_statistics::StatsWindowPlugin,
                panel_equipments::EquipmentPanelPlugin,
                popup_info::PopupInfoPlugin,
            ));
        }
    }
}
