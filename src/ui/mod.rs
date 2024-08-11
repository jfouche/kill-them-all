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
    }
}

// #[inline]
// pub fn hsizer() -> NodeBundle {
//     NodeBundle {
//         style: Style {
//             flex_direction: FlexDirection::Row,
//             align_items: AlignItems::Center,
//             ..default()
//         },
//         ..default()
//     }
// }

#[inline]
pub fn vsizer() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}
