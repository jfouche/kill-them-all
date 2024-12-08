mod button;
mod popup;
mod progressbar;

pub use button::*;
pub use popup::*;
pub use progressbar::*;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(progressbar_plugin)
            .add(button_plugin)
            .add(popup_plugin)
    }
}
