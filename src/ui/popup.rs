use super::vsizer;
use bevy::prelude::*;

#[inline]
fn popup() -> NodeBundle {
    let vsizer = vsizer();
    NodeBundle {
        background_color: Color::srgb(0.25, 0.25, 0.25).into(),
        border_color: Color::BLACK.into(),
        style: Style {
            border: UiRect::all(Val::Px(2.0)),
            width: Val::Percent(35.0),
            margin: UiRect::all(Val::Auto),
            padding: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(7.0),
            },
            ..vsizer.style
        },
        ..vsizer
    }
}

#[inline]
fn popup_title_bar() -> NodeBundle {
    NodeBundle {
        background_color: Color::srgb(0.1, 0.1, 0.1).into(),
        style: Style {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(2.0)),
            ..Default::default()
        },
        ..Default::default()
    }
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

#[inline]
pub fn popup_text_content(content: impl Into<String>) -> TextBundle {
    TextBundle::from_section(
        content,
        TextStyle {
            font_size: 24.0,
            color: Color::WHITE,
            ..Default::default()
        },
    )
    .with_style(Style {
        margin: UiRect::all(Val::Px(7.0)),
        ..Default::default()
    })
}

pub fn spawn_popup(
    mut commands: Commands,
    title: impl Into<String>,
    bundle: impl Bundle,
    spawn_content: impl FnOnce(&mut ChildBuilder),
) -> Entity {
    commands
        .spawn((popup(), bundle))
        .with_children(|menu| {
            menu.spawn(popup_title_bar()).with_children(|title_bar| {
                title_bar.spawn(popup_title(title));
            });
            spawn_content(menu);
        })
        .id()
}
