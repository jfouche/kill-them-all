use bevy::prelude::*;
use crate::{components::*, resources::ScoreResource};

#[derive(Component)]
struct ScoreText;

pub struct TopMenuPlugin;

impl Plugin for TopMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_top_menu)
            .add_system(update_score);
    }
}

fn init_top_menu(mut commands: Commands, asset_server: Res<AssetServer>)
{
    // Score
    commands.spawn_bundle(
        TextBundle::from_sections( [
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                }),
                TextSection::from_style(
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    }),
                    ]
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_CENTER)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                ..default()
            },
            ..default()
        })
    ).insert(ScoreText);
}

fn update_score(score: Res<ScoreResource>, mut q_text: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = q_text.get_single_mut() {
        text.sections[1].value = format!("{}", score.0);
    }
}