pub mod button;
pub mod popup;
pub mod progressbar;

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
            .add(progressbar::ProgressBarPlugin)
            .add(button::button_plugin)
    }
}
