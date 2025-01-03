use crate::{components::*, in_game::GameRunningSet};
use bevy::prelude::*;

///
/// A panel that shows the players characteristics
///
#[derive(Component)]
#[require(
    Name(|| Name::new("CharacteristicsPanel")),
    Node(|| Node {
        position_type: PositionType::Absolute,
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
        left: Val::Px(0.),
        bottom: Val::Px(0.),
        min_width: Val::Px(200.),
        border: UiRect::all(Val::Px(1.)),
        ..Default::default()
    }),
    BorderColor(|| BorderColor(Color::BLACK)),
    BackgroundColor(|| BackgroundColor(Color::srgba(0.5, 0.3, 0.3, 0.8)))
)]
pub struct CharacteristicsPanel;

///
/// label node for a characteristic
///
#[derive(Component)]
#[require(
    Text,
    TextFont(|| TextFont::from_font_size(10.)),
    TextColor(|| TextColor(Color::BLACK)),
    TextLayout(|| TextLayout::new_with_justify(JustifyText::Right)),
)]
struct CharacteristicLabel;

///
/// Value node for a characteristic
///
#[derive(Component)]
#[require(
    Text,
    TextFont(|| TextFont::from_font_size(10.)),
    TextColor(|| TextColor(Color::BLACK)),
)]
struct CharacteristicValue;

pub struct CharacteristicsPanelPlugin;

impl Plugin for CharacteristicsPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_or_despawn_panel,
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
        );
    }
}

fn spawn_or_despawn_panel(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    panels: Query<Entity, With<CharacteristicsPanel>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyC) {
        return;
    }
    warn!("spawn_or_despawn_panel");

    if let Ok(entity) = panels.get_single() {
        commands.entity(entity).despawn_recursive();
    } else {
        commands.spawn(CharacteristicsPanel).with_children(|panel| {
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
    if let Ok(value) = players.get_single() {
        for mut text in &mut texts {
            *text = Text(value.to_string());
        }
    }
}
