use crate::{
    components::{
        affix::{
            Armour, IncreaseAttackSpeed, IncreaseDamage, IncreaseMovementSpeed, LifeRegen,
            MoreDamage, PierceChance,
        },
        character::MaxLife,
        despawn_all,
        player::Player,
    },
    in_game::{GameRunningSet, GameState},
};
use bevy::prelude::*;

///
/// A window that shows the players statistics
///
#[derive(Component)]
#[require(
    Name(|| Name::new("StatisticsWindow")),
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
pub struct StatisticsWindow;

///
/// A panel that shows the players statistics
///
#[derive(Component)]
#[require(
    Name(|| Name::new("StatisticsPanel")),
    Node(|| Node {
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
        ..Default::default()
    }),
)]
pub struct StatisticsPanel;

///
/// label node for a statistic
///
#[derive(Component)]
#[require(
    Text,
    TextFont(|| TextFont::from_font_size(FONT_SIZE)),
    TextColor(|| TextColor(FONT_COLOR)),
    TextLayout(|| TextLayout::new_with_justify(JustifyText::Right)),
)]
struct StatLabel;

///
/// Value node for a statistic
///
#[derive(Component)]
#[require(
    Text,
    TextFont(|| TextFont::from_font_size(FONT_SIZE)),
    TextColor(|| TextColor(FONT_COLOR)),
)]
struct StatValue;

const BACKGROUND_COLOR: Color = Color::srgba(0.5, 0.3, 0.3, 0.8);
const FONT_SIZE: f32 = 10.;
const FONT_COLOR: Color = Color::BLACK;

pub struct StatsWindowPlugin;

impl Plugin for StatsWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::InGame), despawn_all::<StatisticsWindow>)
            .add_systems(
                Update,
                (
                    spawn_or_despawn_window,
                    update_stat::<Armour>,
                    update_stat::<MaxLife>,
                    update_stat::<LifeRegen>,
                    update_stat::<IncreaseMovementSpeed>,
                    update_stat::<IncreaseAttackSpeed>,
                    update_stat::<PierceChance>,
                    update_stat::<MoreDamage>,
                    update_stat::<IncreaseDamage>,
                )
                    .in_set(GameRunningSet::UserInput),
            )
            .add_observer(create_panel);
    }
}

fn spawn_or_despawn_window(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<Entity, With<StatisticsWindow>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyC) {
        return;
    }

    if let Ok(entity) = windows.get_single() {
        commands.entity(entity).despawn_recursive();
    } else {
        commands.spawn(StatisticsWindow).with_child(StatisticsPanel);
    }
}

fn create_panel(trigger: Trigger<OnAdd, StatisticsPanel>, mut commands: Commands) {
    commands.entity(trigger.entity()).with_children(|panel| {
        spawn_stat::<Armour>(panel, "Armour :");
        spawn_stat::<MaxLife>(panel, "Maximum life :");
        spawn_stat::<LifeRegen>(panel, "Life regeneration :");
        spawn_stat::<IncreaseMovementSpeed>(panel, "Movement speed :");
        spawn_stat::<IncreaseAttackSpeed>(panel, "Attack speed :");
        spawn_stat::<PierceChance>(panel, "Pierce chance :");
        spawn_stat::<MoreDamage>(panel, "More damage :");
        spawn_stat::<IncreaseDamage>(panel, "Increase damage :");
    });
}

fn spawn_stat<T: std::fmt::Display + Component + Default>(
    commands: &mut ChildBuilder,
    label: &str,
) {
    commands.spawn((StatLabel, Text(label.into())));
    commands.spawn((StatValue, T::default()));
}

fn update_stat<T: Component + std::fmt::Display>(
    players: Query<&T, With<Player>>,
    mut texts: Query<&mut Text, (With<StatValue>, With<T>)>,
) {
    for stat in &players {
        for mut text in &mut texts {
            **text = stat.to_string();
        }
    }
}
