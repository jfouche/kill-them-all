mod button;
mod popup;
mod progressbar;

pub use button::{button_keyboard_nav, MyButton, SelectedOption};
pub use popup::Popup;
pub use progressbar::{ProgressBar, ProgressBarColor};

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(progressbar::ProgressBarPlugin)
            .add(button::button_plugin)
            .add(popup::popup_plugin)
    }
}
