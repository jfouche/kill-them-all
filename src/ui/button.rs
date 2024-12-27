use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.35, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.55, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

const BUTTON_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

///
/// Component to spawn a button
///
#[derive(Component, Default, Clone)]
#[component(on_add = create_button)]
#[require(
    Button,
    Node(MyButton::default_node),
    BackgroundColor(|| BackgroundColor(NORMAL_BUTTON)),
    BorderColor(|| BorderColor(Color::BLACK))
)]
pub struct MyButton {
    text: Option<Text>,
    image: Option<ImageNode>,
}

impl MyButton {
    pub fn new(label: impl Into<String>) -> Self {
        MyButton {
            text: Some(Text(label.into())),
            image: None,
        }
    }

    pub fn from_image(image: ImageNode) -> Self {
        MyButton {
            text: None,
            image: Some(image),
        }
    }

    // pub fn with_image(mut self, image: ImageNode) -> Self {
    //     self.image = Some(image);
    //     self
    // }

    #[inline]
    pub fn default_node() -> Node {
        Node {
            width: Val::Px(180.0),
            height: Val::Px(50.0),
            margin: UiRect::all(Val::Px(10.0)),
            padding: UiRect::all(Val::Px(2.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        }
    }
}

fn create_button(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    let button = world
        .get::<MyButton>(entity)
        .expect("Added MyButton")
        .clone();

    let have_image = button.image.is_some();

    world.commands().entity(entity).with_children(|parent| {
        if let Some(image) = button.image {
            parent.spawn(image);
        }
        if let Some(text) = button.text {
            parent.spawn((MyButtonText, text));
        }
    });

    if have_image {
        world.get_mut::<Node>(entity).expect("Node").height = Val::Auto;
    }
}

#[derive(Component)]
#[require(
    Text,
    TextFont(|| TextFont::from_font_size(14.)),
    TextColor(|| TextColor(BUTTON_TEXT_COLOR)),
    TextLayout(|| TextLayout::new_with_justify(JustifyText::Center))
)]
struct MyButtonText;

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
        (
            color_buttons,
            color_selected_buttons,
            color_deselected_buttons,
        ),
    );
}

// This system handles changing all buttons color based on mouse interaction
fn color_buttons(
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

fn color_selected_buttons(
    mut query: Query<&mut BackgroundColor, (With<Button>, Added<SelectedOption>)>,
) {
    for mut color in &mut query {
        *color = PRESSED_BUTTON.into();
    }
}

fn color_deselected_buttons(
    mut buttons: Query<&mut BackgroundColor, With<Button>>,
    mut removed: RemovedComponents<SelectedOption>,
) {
    for entity in removed.read() {
        if let Ok(mut color) = buttons.get_mut(entity) {
            *color = NORMAL_BUTTON.into();
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
