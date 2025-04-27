use bevy::prelude::*;

#[derive(Component, Default)]
#[require(
    Node = Popup::default_node(),
    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
    BorderColor(Color::BLACK)
)]
pub struct Popup;

impl Popup {
    #[inline]
    pub fn default_node() -> Node {
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            width: Val::Percent(40.0),
            margin: UiRect::all(Val::Auto),
            padding: UiRect::bottom(Val::Px(7.0)),
            ..Default::default()
        }
    }
}

#[derive(Component, Default)]
pub struct PopupTitle;

impl PopupTitle {
    pub fn bundle(title: impl Into<String>) -> impl Bundle {
        (
            Name::new("PopupTitle"),
            Node {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            children![
                Text(title.into()),
                TextFont::from_font_size(32.),
                TextColor(Color::srgb(0.72, 0.72, 0.72))
            ],
        )
    }
}
