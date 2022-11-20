use crate::prelude::*;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct LifeText;

#[derive(Component)]
struct SpeedText;

pub struct TopMenuPlugin;

impl Plugin for TopMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_score)
            .add_startup_system(spawn_life)
            .add_startup_system(spawn_speed)
            .add_system(update_score)
            .add_system(update_life)
            .add_system(update_speed);
    }
}

fn spawn_score(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                }),
            ])
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(ScoreText);
}

fn spawn_life(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Life: ",
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                }),
            ])
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(300.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(LifeText);
}

fn spawn_speed(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Speed: ",
                    TextStyle {
                        font: font.clone(),
                        font_size: 10.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: font.clone(),
                    font_size: 10.0,
                    color: Color::WHITE,
                }),
            ])
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(600.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(SpeedText);
}

fn update_score(score: Res<ScoreResource>, mut q_text: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = q_text.get_single_mut() {
        text.sections[1].value = format!("{}", score.0);
    }
}

fn update_life(q_player: Query<&Life, With<Player>>, mut q_text: Query<&mut Text, With<LifeText>>) {
    if let Ok(mut text) = q_text.get_single_mut() {
        if let Ok(life) = q_player.get_single() {
            text.sections[1].value = format!("{}", life.value());
        }
    }
}

fn update_speed(
    q_player: Query<&Speed, With<Player>>,
    mut q_text: Query<&mut Text, With<SpeedText>>,
) {
    if let Ok(mut text) = q_text.get_single_mut() {
        if let Ok(speed) = q_player.get_single() {
            text.sections[1].value = format!("{}", speed.0);
        }
    }
}
