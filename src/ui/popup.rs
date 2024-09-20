use super::{vsizer, SpawnImpl};
use bevy::{ecs::system::EntityCommands, prelude::*};

pub trait SpawnPopup {
    fn spawn_popup(&mut self, title: impl Into<String>, bundle: impl Bundle) -> EntityCommands;
}

impl<T> SpawnPopup for T
where
    T: SpawnImpl,
{
    fn spawn_popup(&mut self, title: impl Into<String>, bundle: impl Bundle) -> EntityCommands {
        let mut e = self.spawn_impl((popup(), bundle));
        e.with_children(|menu| {
            menu.spawn(popup_title_bar()).with_children(|title_bar| {
                title_bar.spawn(popup_title(title));
            });
        });
        e
    }
}

#[inline]
fn popup() -> NodeBundle {
    let vsizer = vsizer();
    NodeBundle {
        background_color: Color::srgb(0.25, 0.25, 0.25).into(),
        border_color: Color::BLACK.into(),
        style: Style {
            border: UiRect::all(Val::Px(2.0)),
            width: Val::Percent(40.0),
            margin: UiRect::all(Val::Auto),
            padding: UiRect::bottom(Val::Px(7.0)),
            ..vsizer.style
        },
        ..vsizer
    }
}

#[inline]
fn popup_title_bar() -> impl Bundle {
    (
        Name::new("Title"),
        NodeBundle {
            background_color: Color::srgb(0.1, 0.1, 0.1).into(),
            style: Style {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}

#[inline]
fn popup_title(title: impl Into<String>) -> TextBundle {
    TextBundle::from_section(
        title.into(),
        TextStyle {
            font_size: 32.0,
            color: Color::srgb(0.72, 0.72, 0.72),
            ..default()
        },
    )
}

// #[inline]
// pub fn popup_text_content(content: impl Into<String>) -> TextBundle {
//     TextBundle::from_section(
//         content,
//         TextStyle {
//             font_size: 24.0,
//             color: Color::WHITE,
//             ..Default::default()
//         },
//     )
//     .with_style(Style {
//         margin: UiRect::all(Val::Px(7.0)),
//         ..Default::default()
//     })
// }
