use super::GameRunningSet;
use super::GameState;
use crate::components::*;
use crate::ui::ProgressBar;
use bevy::color::palettes::css::{GOLD, RED};
use bevy::prelude::*;

pub struct TopMenuPlugin;

impl Plugin for TopMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            (spawn_life_bar, spawn_xp_bar, spawn_round, spawn_score),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all::<Hud>)
        .add_systems(
            Update,
            (
                update_score,
                update_round,
                update_life_bar,
                update_xp_bar,
                update_life_bar_on_death,
            )
                .in_set(GameRunningSet::EntityUpdate),
        );
    }
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct RoundText;

#[derive(Component)]
struct LifeBar;

#[derive(Component)]
struct ExperienceBar;

fn spawn_life_bar(mut commands: Commands) {
    commands.spawn((
        (Hud, LifeBar),
        Name::new("HUD - LifeBar"),
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(50.),
                top: Val::Px(20.),
                width: Val::Px(200.),
                height: Val::Px(30.),
                ..Default::default()
            },
            ..Default::default()
        },
        ProgressBar::new(0.0, 100.0, 60.0).with_colors(Color::BLACK, RED.into()),
    ));
}

fn spawn_xp_bar(mut commands: Commands) {
    commands.spawn((
        (Hud, ExperienceBar),
        Name::new("HUD - ExperienceBar"),
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(50.),
                top: Val::Px(20.),
                width: Val::Px(200.),
                height: Val::Px(30.),
                ..Default::default()
            },
            ..Default::default()
        },
        ProgressBar::new(0.0, 100.0, 0.0).with_colors(Color::BLACK, GOLD.into()),
    ));
}

fn spawn_round(mut commands: Commands) {
    let text_style = TextStyle {
        font_size: 20.0,
        color: Color::WHITE,
        ..Default::default()
    };
    commands.spawn((
        (Hud, RoundText),
        Name::new("HUD - Round"),
        TextBundle::from_sections([
            TextSection::new("Round: ", text_style.clone()),
            TextSection::from_style(text_style),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(100.),
            ..Default::default()
        }),
    ));
}

fn spawn_score(mut commands: Commands) {
    let text_style = TextStyle {
        font_size: 20.0,
        color: Color::WHITE,
        ..Default::default()
    };
    commands.spawn((
        (Hud, ScoreText),
        Name::new("HUD - Score"),
        TextBundle::from_sections([
            TextSection::new("Score: ", text_style.clone()),
            TextSection::from_style(text_style),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(100.),
            ..Default::default()
        }),
    ));
}

fn update_score(score: Res<ScoreResource>, mut q_text: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = q_text.get_single_mut() {
        text.sections[1].value = format!("{}", score.0);
    }
}

fn update_round(round: Res<Round>, mut q_text: Query<&mut Text, With<RoundText>>) {
    if let Ok(mut text) = q_text.get_single_mut() {
        text.sections[1].value = format!("{}", round.level());
    }
}

fn update_life_bar(
    q_player: Query<&Life, (With<Player>, Changed<Life>)>,
    mut q_bar: Query<&mut ProgressBar, With<LifeBar>>,
) {
    if let Ok(mut progressbar) = q_bar.get_single_mut() {
        if let Ok(life) = q_player.get_single() {
            progressbar.set_range(0.0, life.max_life() as f32);
            progressbar.set_value(life.life() as f32);
        }
    }
}

fn update_xp_bar(
    q_player: Query<&Experience, With<Player>>,
    mut q_bar: Query<&mut ProgressBar, With<ExperienceBar>>,
) {
    if let Ok(mut progressbar) = q_bar.get_single_mut() {
        if let Ok(xp) = q_player.get_single() {
            let (min, max) = xp.get_current_level_min_max_exp();
            progressbar.set_range(min as f32, max as f32);
            progressbar.set_value(xp.current() as f32);
        }
    }
}

fn update_life_bar_on_death(
    mut player_death_events: EventReader<PlayerDeathEvent>,
    mut q_bar: Query<&mut ProgressBar, With<LifeBar>>,
) {
    if let Ok(mut progressbar) = q_bar.get_single_mut() {
        for _ in player_death_events.read() {
            progressbar.set_value(0.0);
        }
    }
}
