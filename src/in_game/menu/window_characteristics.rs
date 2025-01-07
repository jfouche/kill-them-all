use crate::{components::*, in_game::GameRunningSet, ui::mouse_over_ui::CaptureMouse};
use bevy::prelude::*;

///
/// A panel that shows the players characteristics
///
#[derive(Component)]
#[require(
    Name(|| Name::new("CharacteristicsWindow")),
    CaptureMouse,
    Node(|| Node {
        position_type: PositionType::Absolute,
        left: Val::Px(0.),
        bottom: Val::Px(0.),
        min_width: Val::Px(200.),
        border: UiRect::all(Val::Px(1.)),
        ..Default::default()
    }),
    BorderColor(|| BorderColor(Color::BLACK)),
    BackgroundColor(|| BackgroundColor(BACKGROUND_COLOR))
)]
pub struct CharacteristicsWindow;

///
/// A panel that shows the players characteristics
///
#[derive(Component)]
#[require(
    Name(|| Name::new("CharacteristicsPanel")),
    Node(|| Node {
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
        ..Default::default()
    }),
)]
pub struct CharacteristicsPanel;

///
/// label node for a characteristic
///
#[derive(Component)]
#[require(
    Text,
    TextFont(|| TextFont::from_font_size(FONT_SIZE)),
    TextColor(|| TextColor(FONT_COLOR)),
    TextLayout(|| TextLayout::new_with_justify(JustifyText::Right)),
)]
struct CharacteristicLabel;

///
/// Value node for a characteristic
///
#[derive(Component)]
#[require(
    Text,
    TextFont(|| TextFont::from_font_size(FONT_SIZE)),
    TextColor(|| TextColor(FONT_COLOR)),
)]
struct CharacteristicValue;

const BACKGROUND_COLOR: Color = Color::srgba(0.5, 0.3, 0.3, 0.8);
const FONT_SIZE: f32 = 10.;
const FONT_COLOR: Color = Color::BLACK;

pub struct CharacteristicsPanelPlugin;

impl Plugin for CharacteristicsPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_or_despawn_window,
                update_characteristic::<Armour>,
                update_characteristic::<MaxLife>,
                update_characteristic::<LifeRegen>,
                update_characteristic::<IncreaseMovementSpeed>,
                update_characteristic::<IncreaseAttackSpeed>,
                update_characteristic::<PierceChance>,
                update_characteristic::<MoreDamage>,
                update_characteristic::<IncreaseDamage>,
            )
                .in_set(GameRunningSet::UserInput),
        )
        .add_observer(create_panel);
    }
}

fn spawn_or_despawn_window(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<Entity, With<CharacteristicsWindow>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyC) {
        return;
    }

    if let Ok(entity) = windows.get_single() {
        commands.entity(entity).despawn_recursive();
    } else {
        commands
            .spawn(CharacteristicsWindow)
            .with_child(CharacteristicsPanel);
    }
}

fn create_panel(trigger: Trigger<OnAdd, CharacteristicsPanel>, mut commands: Commands) {
    commands.entity(trigger.entity()).with_children(|panel| {
        spawn_characteristic::<Armour>(panel, "Armour :");
        spawn_characteristic::<MaxLife>(panel, "Maximum life :");
        spawn_characteristic::<LifeRegen>(panel, "Life regeneration :");
        spawn_characteristic::<IncreaseMovementSpeed>(panel, "Movement speed :");
        spawn_characteristic::<IncreaseAttackSpeed>(panel, "Attack speed :");
        spawn_characteristic::<PierceChance>(panel, "Pierce chance :");
        spawn_characteristic::<MoreDamage>(panel, "More damage :");
        spawn_characteristic::<IncreaseDamage>(panel, "Increase damage :");
    });
}

fn spawn_characteristic<T: std::fmt::Display + Component + Default>(
    commands: &mut ChildBuilder,
    label: &str,
) {
    commands.spawn((CharacteristicLabel, Text(label.into())));
    commands.spawn((CharacteristicValue, T::default()));
}

fn update_characteristic<T: Component + std::fmt::Display>(
    players: Query<&T, With<Player>>,
    mut texts: Query<&mut Text, (With<CharacteristicValue>, With<T>)>,
) {
    for characteritic in &players {
        for mut text in &mut texts {
            **text = characteritic.to_string();
        }
    }
}
