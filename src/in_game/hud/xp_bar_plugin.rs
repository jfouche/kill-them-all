use super::Hud;
use crate::{
    components::player::{Experience, Player},
    schedule::{GameRunningSet, GameState},
    ui::progressbar::{ProgressBar, ProgressBarColor},
};
use bevy::{color::palettes::css::GOLD, prelude::*};

pub struct ExperienceBarPlugin;

impl Plugin for ExperienceBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_xp_bar)
            .add_systems(Update, update_xp_bar.in_set(GameRunningSet::EntityUpdate));
    }
}

#[derive(Component)]
#[require(
    Hud,
    Name(|| Name::new("HUD - ExperienceBar")),
    Node(|| Node {
        position_type: PositionType::Absolute,
        right: Val::Px(50.),
        top: Val::Px(20.),
        width: Val::Px(300.),
        height: Val::Px(20.),
        border: UiRect::all(Val::Px(2.)),
        ..Default::default()
    }),
    BackgroundColor(|| BackgroundColor(Color::BLACK)),
    BorderColor(|| BorderColor(Color::BLACK)),
    ProgressBar,
    ProgressBarColor(|| ProgressBarColor(GOLD.into()))
)]
struct ExperienceBar;

fn spawn_xp_bar(mut commands: Commands) {
    commands.spawn(ExperienceBar);
}

fn update_xp_bar(
    q_player: Query<&Experience, With<Player>>,
    mut q_bar: Query<&mut ProgressBar, With<ExperienceBar>>,
) {
    if let Ok(mut progressbar) = q_bar.get_single_mut() {
        if let Ok(xp) = q_player.get_single() {
            let (min, max) = xp.get_current_level_min_max_exp();
            progressbar.min = min as f32;
            progressbar.max = max as f32;
            progressbar.value = xp.current() as f32;
        }
    }
}
