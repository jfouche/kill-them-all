mod button;
mod popup;
mod progressbar;

pub use button::*;
pub use popup::*;
pub use progressbar::*;

use crate::prelude::*;
use bevy::app::PluginGroupBuilder;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(progressbar::plugin)
    }
}

pub fn fullscreen_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }
}

pub fn centered_style() -> Style {
    Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..fullscreen_style()
    }
}

pub fn centered() -> NodeBundle {
    NodeBundle {
        style: centered_style(),
        ..default()
    }
}

pub fn hsizer() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}

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
