mod button;
mod popup;
mod progressbar;

pub use button::{button_keyboard_nav, MyButton, SelectedOption};
pub use popup::Popup;
pub use progressbar::{ProgressBar, ProgressBarColor};

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
            .add(popup::popup_plugin)
    }
}
