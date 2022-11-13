use bevy::prelude::*;
use crate::{resources::ScoreResource, components::*};

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct LifeText;

pub struct TopMenuPlugin;

impl Plugin for TopMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_top_menu)
            .add_system(update_score)
            .add_system(update_life);
    }
}

fn init_top_menu(mut commands: Commands, asset_server: Res<AssetServer>)
{
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
//    commands.insert_resource(UiFont(handle));

    // Score
    commands.spawn_bundle(
        TextBundle::from_sections( [
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                }),
                TextSection::from_style(
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    }),
                    ]
        ) 
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                ..default()
            },
            ..default()
        })
    ).insert(ScoreText);


    // Life
    commands.spawn_bundle(
        TextBundle::from_sections( [
            TextSection::new(
                "Life: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                }),
                TextSection::from_style(
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    }),
                    ]
        ) 
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(300.0),
                ..default()
            },
            ..default()
        })
    ).insert(LifeText);

}

fn update_score(score: Res<ScoreResource>, mut q_text: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = q_text.get_single_mut() {
        text.sections[1].value = format!("{}", score.0);
    }
}

fn update_life(q_player: Query<&Life, With<Player>>, mut q_text: Query<&mut Text, With<LifeText>>) {
    if let Ok(mut text) = q_text.get_single_mut() {
        if let Ok(life) = q_player.get_single() {
            text.sections[1].value = format!("{}", life.0);
        }
    }
}