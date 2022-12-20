use crate::prelude::*;
use crate::ui::{Percent, ProgressBarBundle, ProgressBarData};

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

impl Percent for Life {
    fn percent(&self) -> f32 {
        100.0 * self.life() as f32 / self.max_life() as f32
    }
}

impl Percent for Experience {
    fn percent(&self) -> f32 {
        let (min, max) = self.get_current_level_min_max_exp();
        100.0 * (self.current() - min) as f32 / (max - min) as f32
    }
}

pub struct TopMenuPlugin;

impl Plugin for TopMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_top_menu)
            .add_system(update_score)
            .add_system(update_round)
            .add_system(update_life_bar)
            .add_system(update_xp_bar)
            .add_system(update_life_on_death)
            .add_system(update_life_bar_on_death);
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
        .insert(ProgressBarBundle::new(
            ProgressBarData::from_size(Size::new(Val::Px(600.0), Val::Px(20.0)))
                .with_colors(Color::BLACK, Color::RED),
        ));
}

fn spawn_xp_bar(parent: &mut ChildBuilder) {
    parent
        .spawn(ExperienceBar)
        .insert(Name::new("Xp bar"))
        .insert(ProgressBarBundle::new(
            ProgressBarData::from_size(Size::new(Val::Px(600.0), Val::Px(20.0)))
                .with_colors(Color::BLACK, Color::GOLD),
        ));
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
    q_player: Query<&Life, With<Player>>,
    mut q_bar: Query<&mut ProgressBarData, With<LifeBar>>,
) {
    if let Ok(mut progressbar) = q_bar.get_single_mut() {
        if let Ok(life) = q_player.get_single() {
            progressbar.set_percent(life.percent());
        }
    }
}
fn update_xp_bar(
    q_player: Query<&Experience, With<Player>>,
    mut q_bar: Query<&mut ProgressBarData, With<ExperienceBar>>,
) {
    if let Ok(mut progressbar) = q_bar.get_single_mut() {
        if let Ok(xp) = q_player.get_single() {
            progressbar.set_percent(xp.percent());
        }
    }
}

fn update_life_on_death(
    mut player_death_events: EventReader<PlayerDeathEvent>,
    mut q_text: Query<&mut Text, With<LifeText>>,
) {
    if let Ok(mut text) = q_text.get_single_mut() {
        for _ in player_death_events.iter() {
            text.sections[1].value = "0".to_string();
        }
    }
}

fn update_life_bar_on_death(
    mut player_death_events: EventReader<PlayerDeathEvent>,
    mut q_bar: Query<&mut ProgressBarData, With<LifeBar>>,
) {
    if let Ok(mut progressbar) = q_bar.get_single_mut() {
        for _ in player_death_events.iter() {
            progressbar.set_percent(0.0);
        }
    }
}
