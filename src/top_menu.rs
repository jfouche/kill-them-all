use crate::prelude::*;
use crate::ui::{ProgressBarBundle, ProgressBarData};

#[derive(Component)]
struct TopMenu;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct LifeText;

#[derive(Component)]
struct LifeBar;

#[derive(Component)]
struct SpeedText;

pub struct TopMenuPlugin;

impl Plugin for TopMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_top_menu)
            .add_system(update_score)
            .add_system(update_life)
            .add_system(update_life_bar)
            .add_system(update_speed);
    }
}

fn spawn_top_menu(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(30.)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..Default::default()
        })
        .insert(TopMenu)
        .insert(Name::new("Top menu"))
        .with_children(|top_menu| {
            let text_style = TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::WHITE,
            };
            // -----------------------------------------
            // LIFE
            top_menu
                .spawn(
                    TextBundle::from_sections([
                        TextSection::new("Life: ", text_style.clone()),
                        TextSection::from_style(text_style.clone()),
                    ])
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(2.)),
                        size: Size::new(Val::Px(180.), Val::Auto),
                        ..Default::default()
                    }),
                )
                .insert(LifeText)
                .insert(Name::new("Life"));

            top_menu
                .spawn(ProgressBarBundle::new(
                    ProgressBarData::from_size(Size::new(Val::Px(600.0), Val::Px(20.0)))
                        .with_colors(Color::BLACK, Color::RED),
                ))
                .insert(LifeBar)
                .insert(Name::new("Life bar"));
            // -----------------------------------------
            // SCORE
            top_menu
                .spawn(TextBundle::from_sections([
                    TextSection::new("Score: ", text_style.clone()),
                    TextSection::from_style(text_style.clone()),
                ]))
                .insert(ScoreText)
                .insert(Name::new("Score"));
        });
}

fn spawn_score(mut commands: Commands, font: Res<UiFont>, query: Query<Entity, Added<TopMenu>>) {
    if let Ok(top_menu_entity) = query.get_single() {
        commands.entity(top_menu_entity).with_children(|top_menu| {
            top_menu
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
                    //                    .with_text_alignment(TextAlignment::TOP_CENTER)
                    .with_style(Style {
                        //                        position_type: PositionType::Absolute,
                        position: UiRect {
                            top: Val::Px(5.0),
                            ..default()
                        },
                        ..default()
                    }),
                )
                .insert(ScoreText)
                .insert(Name::new("Score"));
        });
    }
}

fn spawn_life(
    mut commands: Commands,
    font: Res<UiFont>,
    query: Query<Entity, (With<TopMenu>, Added<TopMenu>)>,
) {
    if let Ok(top_menu_entity) = query.get_single() {
        let style = TextStyle {
            font: font.clone(),
            font_size: 20.0,
            color: Color::WHITE,
        };
        commands.entity(top_menu_entity).with_children(|top_menu| {
            top_menu
                .spawn(
                    TextBundle::from_sections([
                        TextSection::new("Life: ", style.clone()),
                        TextSection::from_style(style),
                    ]), // .with_text_alignment(TextAlignment::TOP_CENTER)
                        // .with_style(Style {
                        //     // position_type: PositionType::Absolute,
                        //     position: UiRect {
                        //         top: Val::Px(5.0),
                        //         left: Val::Px(300.0),
                        //         ..default()
                        //     },
                        //     ..default()
                        // }),
                )
                .insert(LifeText)
                .insert(Name::new("Life"));

            top_menu
                .spawn(ProgressBarBundle::new(
                    ProgressBarData::from_size(Size::new(Val::Px(300.0), Val::Px(30.0)))
                        .with_colors(Color::BLACK, Color::RED),
                ))
                .insert(LifeBar)
                .insert(Name::new("Life bar"));
        });
    }
}

fn spawn_speed(
    mut commands: Commands,
    font: Res<UiFont>,
    query: Query<Entity, (With<TopMenu>, Added<TopMenu>)>,
) {
    if let Ok(top_menu_entity) = query.get_single() {
        let style = TextStyle {
            font: font.clone(),
            font_size: 20.0,
            color: Color::WHITE,
        };
        commands.entity(top_menu_entity).with_children(|top_menu| {
            top_menu
                .spawn(TextBundle::from_sections([
                    TextSection::new("Speed: ", style.clone()),
                    TextSection::from_style(style),
                ]))
                .insert(SpeedText);
        });
    }
}

fn update_score(score: Res<ScoreResource>, mut q_text: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = q_text.get_single_mut() {
        text.sections[1].value = format!("{}", score.0);
    }
}

fn update_life(q_player: Query<&Life, With<Player>>, mut q_text: Query<&mut Text, With<LifeText>>) {
    if let Ok(mut text) = q_text.get_single_mut() {
        if let Ok(life) = q_player.get_single() {
            text.sections[1].value = format!("{}", life.life());
        }
    }
}

fn update_life_bar(
    q_player: Query<&Life, With<Player>>,
    mut q_bar: Query<&mut ProgressBarData, With<LifeBar>>,
) {
    if let Ok(mut progressbar) = q_bar.get_single_mut() {
        if let Ok(life) = q_player.get_single() {
            progressbar.set_percent(life.percent())
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
