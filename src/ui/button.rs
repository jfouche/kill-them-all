use bevy::prelude::*;

const NORMAL_BUTTON_BG_COLOR: Color = Color::srgb(0.15, 0.35, 0.15);
const HOVERED_BUTTON_BG_COLOR: Color = Color::srgb(0.25, 0.55, 0.25);
const PRESSED_BUTTON_BG_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);

const BUTTON_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BIG_BUTTON_FONT_SIZE: f32 = 18.;
const SMALL_BUTTON_FONT_SIZE: f32 = 12.;

pub fn big_button_node() -> Node {
    Node {
        width: Val::Px(180.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    }
}

pub fn small_button_node() -> Node {
    Node {
        width: Val::Px(120.0),
        height: Val::Px(35.0),
        margin: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    }
}

#[derive(Component, Reflect)]
#[require(BackgroundColor)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: NORMAL_BUTTON_BG_COLOR,
            hovered: HOVERED_BUTTON_BG_COLOR,
            pressed: PRESSED_BUTTON_BG_COLOR,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum ButtonKind {
    Big,
    Small,
}

///
/// Component to spawn a button with text
///
#[derive(Component, Clone)]
#[require(Button, ButtonColors, Node, BorderColor(Color::BLACK))]
pub struct TextButton {
    pub text: String,
    pub kind: ButtonKind,
}

impl TextButton {
    pub fn big(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: ButtonKind::Big,
        }
    }

    #[allow(dead_code)]
    pub fn small(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: ButtonKind::Small,
        }
    }
}

pub fn button_plugin(app: &mut App) {
    app.register_type::<ButtonColors>()
        .add_systems(Update, color_buttons)
        .add_observer(create_text_button)
        .add_observer(init_color);
}

fn create_text_button(
    trigger: Trigger<OnAdd, TextButton>,
    mut commands: Commands,
    mut buttons: Query<(&TextButton, &mut Node, &mut BackgroundColor, &ButtonColors)>,
) {
    let (button, mut node, mut bgcolor, colors) =
        buttons.get_mut(trigger.target()).expect("Added TextButton");

    *bgcolor = colors.normal.into();

    *node = match button.kind {
        ButtonKind::Big => big_button_node(),
        ButtonKind::Small => small_button_node(),
    };

    let font_size = match button.kind {
        ButtonKind::Big => BIG_BUTTON_FONT_SIZE,
        ButtonKind::Small => SMALL_BUTTON_FONT_SIZE,
    };
    commands.spawn((
        Text(button.text.clone()),
        TextColor(BUTTON_TEXT_COLOR),
        TextFont::from_font_size(font_size),
        TextLayout::new_with_justify(JustifyText::Center),
        ChildOf(trigger.target()),
    ));
    // TODO: add observers to change color ?
}

fn init_color(
    trigger: Trigger<OnAdd, ButtonColors>,
    mut buttons: Query<(&mut BackgroundColor, &ButtonColors)>,
) {
    if let Ok((mut bgcolor, colors)) = buttons.get_mut(trigger.target()) {
        *bgcolor = colors.normal.into();
    }
}

// This system handles changing all buttons color based on mouse interaction
fn color_buttons(
    mut query: Query<(&Interaction, &mut BackgroundColor, &ButtonColors), Changed<Interaction>>,
) {
    for (interaction, mut bgcolor, colors) in &mut query {
        *bgcolor = match *interaction {
            Interaction::Pressed => colors.pressed.into(),
            Interaction::Hovered => colors.hovered.into(),
            Interaction::None => colors.normal.into(),
        }
    }
}
