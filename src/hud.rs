use crate::prelude::*;
use crate::ui::ProgressBar;
use bevy::color::palettes::css::{GOLD, GRAY, RED};

#[derive(Component)]
struct TopMenu;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct RoundText;

#[derive(Component)]
struct LifeText;

#[derive(Component)]
struct LifeBar;

#[derive(Component)]
struct ExperienceBar;

// impl Percent for Life {
//     fn percent(&self) -> f32 {
//         100.0 * self.life() as f32 / self.max_life() as f32
//     }
// }

// impl Percent for Experience {
//     fn percent(&self) -> f32 {
//         let (min, max) = self.get_current_level_min_max_exp();
//         100.0 * (self.current() - min) as f32 / (max - min) as f32
//     }
// }

pub struct TopMenuPlugin;

impl Plugin for TopMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_top_menu).add_systems(
            Update,
            (
                update_score,
                update_round,
                update_life_bar,
                update_xp_bar,
                update_life_on_death,
                update_life_bar_on_death,
            ),
        );
    }
}

fn spawn_top_menu(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(30.),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: GRAY.into(),
            ..Default::default()
        })
        .insert(TopMenu)
        .insert(Name::new("Top menu"))
        .with_children(|top_menu| {
            spawn_life_bar(top_menu);
            spawn_xp_bar(top_menu);
            spawn_round(top_menu, font.clone());
            spawn_score(top_menu, font.clone());
        });
}

fn spawn_life_bar(parent: &mut ChildBuilder) {
    parent
        .spawn(LifeBar)
        .insert(Name::new("Life bar"))
        .insert(NodeBundle {
            style: Style {
                width: Val::Px(600.0),
                height: Val::Px(20.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ProgressBar::new(0.0, 100.0, 60.0).with_colors(Color::BLACK, RED.into()));
}

fn spawn_xp_bar(parent: &mut ChildBuilder) {
    parent
        .spawn(ExperienceBar)
        .insert(Name::new("Xp bar"))
        .insert(NodeBundle {
            style: Style {
                width: Val::Px(600.0),
                height: Val::Px(20.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ProgressBar::new(0.0, 100.0, 0.0).with_colors(Color::BLACK, GOLD.into()));
}

fn spawn_round(parent: &mut ChildBuilder, font: Handle<Font>) {
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::WHITE,
    };
    parent
        .spawn(RoundText)
        .insert(Name::new("Round"))
        .insert(TextBundle::from_sections([
            TextSection::new("Round: ", text_style.clone()),
            TextSection::from_style(text_style),
        ]));
}

fn spawn_score(parent: &mut ChildBuilder, font: Handle<Font>) {
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::WHITE,
    };
    parent
        .spawn(ScoreText)
        .insert(Name::new("Score"))
        .insert(TextBundle::from_sections([
            TextSection::new("Score: ", text_style.clone()),
            TextSection::from_style(text_style),
        ]));
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

fn update_life_on_death(
    mut player_death_events: EventReader<PlayerDeathEvent>,
    mut q_text: Query<&mut Text, With<LifeText>>,
) {
    if let Ok(mut text) = q_text.get_single_mut() {
        for _ in player_death_events.read() {
            text.sections[1].value = "0".to_string();
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
