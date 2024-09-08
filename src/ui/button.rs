use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.35, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.55, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

const BUTTON_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[inline]
pub fn button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: button_style(),
        background_color: NORMAL_BUTTON.into(),
        border_color: Color::BLACK.into(),
        ..default()
    }
}

#[inline]
pub fn button_style() -> Style {
    Style {
        width: Val::Px(180.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(10.0)),
        padding: UiRect::all(Val::Px(2.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    }
}

#[inline]
pub fn button_text_style() -> TextStyle {
    TextStyle {
        font_size: 18.0,
        color: BUTTON_TEXT_COLOR,
        ..default()
    }
}

#[inline]
pub fn button_text(text: impl Into<String>) -> TextBundle {
    TextBundle::from_section(text, button_text_style()).with_text_justify(JustifyText::Center)
}

pub fn spawn_button(commands: &mut ChildBuilder, label: impl Into<String>, bundle: impl Bundle) {
    commands
        .spawn((button_bundle(), bundle))
        .with_children(|parent| {
            parent.spawn(button_text(label));
        });
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
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct SelectedOption;

pub fn button_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (button_interractions, button_selected, button_deselected),
    );
}

// This system handles changing all buttons color based on mouse interaction
pub fn button_interractions(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

/// System to handle keyboard on a menu
///
/// This system should be run before the system that handle the action,
/// because it uses [Interaction::Pressed] to inform the action key is pressed
pub fn button_keyboard_nav<T, N>(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    sel_buttons: Query<(Entity, &T), With<SelectedOption>>,
    buttons: Query<(Entity, &T), With<Button>>,
    nav: Res<N>,
) where
    T: Component + PartialEq + Copy,
    N: ButtonNav<T> + Resource,
{
    for (sel_entity, sel_btn) in &sel_buttons {
        if keys.just_pressed(KeyCode::Enter) {
            commands.entity(sel_entity).insert(Interaction::Pressed);
        }
        if keys.just_pressed(KeyCode::ArrowUp) {
            if let Some(top) = nav.up(*sel_btn) {
                if let Some((e, _v)) = buttons.iter().find(|(_e, v)| **v == top) {
                    commands.entity(sel_entity).remove::<SelectedOption>();
                    commands.entity(e).insert(SelectedOption);
                }
            }
        }
        if keys.just_pressed(KeyCode::ArrowDown) {
            if let Some(top) = nav.down(*sel_btn) {
                if let Some((e, _v)) = buttons.iter().find(|(_e, v)| **v == top) {
                    commands.entity(sel_entity).remove::<SelectedOption>();
                    commands.entity(e).insert(SelectedOption);
                }
            }
        }
    }
}

fn button_selected(mut query: Query<&mut BackgroundColor, (With<Button>, Added<SelectedOption>)>) {
    for mut color in &mut query {
        *color = PRESSED_BUTTON.into();
    }
}

fn button_deselected(
    mut buttons: Query<&mut BackgroundColor, With<Button>>,
    mut removed: RemovedComponents<SelectedOption>,
) {
    for entity in removed.read() {
        if let Ok(mut color) = buttons.get_mut(entity) {
            *color = NORMAL_BUTTON.into();
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
