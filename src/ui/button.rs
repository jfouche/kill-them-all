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

pub trait ButtonNav<T> {
    fn up(&self, current: T) -> Option<T>;
    fn down(&self, current: T) -> Option<T>;
}

impl<T> ButtonNav<T> for [T]
where
    T: PartialEq + Copy,
{
    fn up(&self, current: T) -> Option<T> {
        let i = self.iter().position(|v| *v == current)?;
        self.get(i.saturating_sub(1)).cloned()
    }

    fn down(&self, current: T) -> Option<T> {
        let i = self.iter().position(|v| *v == current)?;
        self.get(i + 1).cloned()
    }
}

impl<S, T, N> ButtonNav<T> for S
where
    S: std::ops::Deref<Target = N>,
    N: ButtonNav<T> + ?Sized,
{
    fn up(&self, current: T) -> Option<T> {
        (**self).up(current)
    }

    fn down(&self, current: T) -> Option<T> {
        (**self).down(current)
    }
}

/// Tag component used to mark which setting is currently selected
#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct SelectedOption;

pub fn button_plugin(app: &mut App) {
    app.register_type::<ButtonColors>()
        .add_systems(
            Update,
            (
                color_buttons,
                color_selected_buttons,
                color_deselected_buttons,
            ),
        )
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
    commands.entity(trigger.target()).with_child((
        Text(button.text.clone()),
        TextColor(BUTTON_TEXT_COLOR),
        TextFont::from_font_size(font_size),
        TextLayout::new_with_justify(JustifyText::Center),
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
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&SelectedOption>,
        ),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut bgcolor, colors, selected) in &mut query {
        *bgcolor = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => colors.pressed.into(),
            (Interaction::Hovered, Some(_)) => colors.pressed.into(),
            (Interaction::Hovered, None) => colors.hovered.into(),
            (Interaction::None, None) => colors.normal.into(),
        }
    }
}

fn color_selected_buttons(
    mut query: Query<(&mut BackgroundColor, &ButtonColors), (With<Button>, Added<SelectedOption>)>,
) {
    for (mut bgcolor, colors) in &mut query {
        *bgcolor = colors.pressed.into();
    }
}

fn color_deselected_buttons(
    mut buttons: Query<(&mut BackgroundColor, &ButtonColors), With<Button>>,
    mut removed: RemovedComponents<SelectedOption>,
) {
    for entity in removed.read() {
        if let Ok((mut bgcolor, colors)) = buttons.get_mut(entity) {
            *bgcolor = colors.normal.into();
        }
    }
}

/// System to handle keyboard on a menu
///
/// This system should be run before the system that handle the action,
/// because it uses [Interaction::Pressed] to inform the action key is pressed
pub fn button_keyboard_nav<N>(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    sel_buttons: Query<Entity, With<SelectedOption>>,
    nav: Res<N>,
) where
    N: ButtonNav<Entity> + Resource,
{
    for sel_entity in &sel_buttons {
        if keys.any_just_pressed([KeyCode::Space, KeyCode::Enter]) {
            commands.entity(sel_entity).insert(Interaction::Pressed);
        }
        if keys.just_pressed(KeyCode::ArrowUp) {
            if let Some(up) = nav.up(sel_entity) {
                commands.entity(sel_entity).remove::<SelectedOption>();
                commands.entity(up).insert(SelectedOption);
            }
        }
        if keys.just_pressed(KeyCode::ArrowDown) {
            if let Some(down) = nav.down(sel_entity) {
                commands.entity(sel_entity).remove::<SelectedOption>();
                commands.entity(down).insert(SelectedOption);
            }
        }
    }
}

// /// This system updates the settings when a new value for a setting is selected, and marks
// /// the button as the one currently selected
// pub fn setting_button<T: Resource + Component + PartialEq + Copy>(
//     interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
//     mut selected_query: Query<(Entity, &mut BackgroundColor), (With<SelectedOption>, With<T>)>,
//     mut commands: Commands,
//     mut setting: ResMut<T>,
// ) {
//     for (interaction, button_setting, entity) in &interaction_query {
//         if *interaction == Interaction::Pressed && *setting != *button_setting {
//             if let Ok((previous_button, mut previous_color)) = selected_query.get_single_mut() {
//                 *previous_color = NORMAL_BUTTON.into();
//                 commands.entity(previous_button).remove::<SelectedOption>();
//             }
//             commands.entity(entity).insert(SelectedOption);
//             *setting = *button_setting;
//         }
//     }
// }
