mod button;
mod popup;
mod progressbar;

pub use button::*;
pub use popup::*;
pub use progressbar::*;

use bevy::app::PluginGroupBuilder;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(progressbar_plugin)
            .add(button_plugin)
    }
}

pub trait SpawnImpl {
    fn spawn_impl(&mut self, bundle: impl Bundle) -> EntityCommands;
}

impl SpawnImpl for Commands<'_, '_> {
    fn spawn_impl(&mut self, bundle: impl Bundle) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl SpawnImpl for ChildBuilder<'_> {
    fn spawn_impl(&mut self, bundle: impl Bundle) -> EntityCommands {
        self.spawn(bundle)
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
pub fn vsizer() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
    }
}
