pub mod button;
pub mod item_location;
pub mod popup;
pub mod progressbar;

pub use plugin::{HSizer, UiPlugins, VSizer};

mod plugin {
    use super::button::button_plugin;
    use super::item_location::ItemLocationPlugin;
    use super::progressbar::ProgressBarPlugin;
    use bevy::app::PluginGroupBuilder;
    use bevy::prelude::*;

    /// Horizontal sizer
    #[derive(Component, Default)]
    #[require(
    Node(|| Node {
            flex_direction: FlexDirection::Row,
            ..Default::default()
        })
    )]
    pub struct HSizer;

    /// Vertical sizer
    #[derive(Component, Default)]
    #[require(
    Node(|| Node {
        flex_direction: FlexDirection::Column,
        ..Default::default()
    })
)]
    pub struct VSizer;

    pub struct UiPlugins;

    impl PluginGroup for UiPlugins {
        fn build(self) -> bevy::app::PluginGroupBuilder {
            PluginGroupBuilder::start::<Self>()
                .add(ProgressBarPlugin)
                .add(button_plugin)
                .add(ItemLocationPlugin)
        }
    }
}
