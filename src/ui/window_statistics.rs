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
    schedule::{GameRunningSet, GameState},
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub struct StatsWindowPlugin;

impl Plugin for StatsWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::InGame), despawn_all::<StatisticsWindow>)
            .add_systems(
                Update,
                (
                    toggle_window.run_if(input_just_pressed(KeyCode::KeyC)),
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
            );
    }
}

///
/// A window that shows the players statistics
///
#[derive(Component)]
struct StatisticsWindow;

const FONT_SIZE: f32 = 10.;
const FONT_COLOR: Color = Color::BLACK;

///
/// label node for a statistic
///
#[derive(Component)]
struct StatLabel;

fn stat_label(text: &str) -> impl Bundle {
    (
        StatLabel,
        Text(text.into()),
        TextFont::from_font_size(FONT_SIZE),
        TextColor(FONT_COLOR),
        TextLayout::new_with_justify(JustifyText::Right),
    )
}

///
/// Value node for a statistic
///
#[derive(Component)]
struct StatValue;

fn stat_value<T: Component + Default>() -> impl Bundle {
    (
        StatValue,
        Text::default(),
        TextFont::from_font_size(FONT_SIZE),
        TextColor(FONT_COLOR),
        T::default(),
    )
}

fn toggle_window(mut commands: Commands, windows: Query<Entity, With<StatisticsWindow>>) {
    if let Ok(entity) = windows.single() {
        commands.entity(entity).despawn();
    } else {
        commands
            .spawn((
                StatisticsWindow,
                Name::new("StatisticsWindow"),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.),
                    bottom: Val::Px(0.),
                    min_width: Val::Px(200.),
                    border: UiRect::all(Val::Px(1.)),
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
                    ..Default::default()
                },
                BorderColor(Color::BLACK),
                BackgroundColor(Color::srgba(0.5, 0.3, 0.3, 0.8)),
            ))
            .with_children(|p| {
                p.spawn(stat_label("Armour:".into()));
                p.spawn(stat_value::<Armour>());
                p.spawn(stat_label("Maximum life:".into()));
                p.spawn(stat_value::<MaxLife>());
                p.spawn(stat_label("Life regeneration:".into()));
                p.spawn(stat_value::<LifeRegen>());
                p.spawn(stat_label("Movement speed:".into()));
                p.spawn(stat_value::<IncreaseMovementSpeed>());
                p.spawn(stat_label("Attack speed:".into()));
                p.spawn(stat_value::<IncreaseAttackSpeed>());
                p.spawn(stat_label("Pierce chance:".into()));
                p.spawn(stat_value::<PierceChance>());
                p.spawn(stat_label("More damage:".into()));
                p.spawn(stat_value::<MoreDamage>());
                p.spawn(stat_label("Increase damage:".into()));
                p.spawn(stat_value::<IncreaseDamage>());
            });
    }
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
