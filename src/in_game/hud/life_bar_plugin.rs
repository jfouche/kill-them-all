use super::Hud;
use crate::{
    components::{
        character::{Life, MaxLife},
        player::{Player, PlayerDeathEvent},
    },
    schedule::{GameRunningSet, GameState},
    ui::progressbar::{ProgressBar, ProgressBarColor},
};
use bevy::{color::palettes::css::RED, prelude::*};

pub struct LifeBarPlugin;

impl Plugin for LifeBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_life_bar)
            .add_systems(Update, update_life_bar.in_set(GameRunningSet::EntityUpdate))
            .add_observer(update_life_bar_on_death);
    }
}

#[derive(Component)]
#[require(
    Hud,
    Name::new("HUD - LifeBar"),
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(50.),
        top: Val::Px(20.),
        width: Val::Px(300.),
        height: Val::Px(20.),
        border: UiRect::all(Val::Px(2.)),
        ..Default::default()
    },
    BackgroundColor(Color::BLACK),
    BorderColor(Color::BLACK),
    ProgressBar,
    ProgressBarColor(RED.into())
)]
struct LifeBar;

fn spawn_life_bar(mut commands: Commands) {
    commands.spawn(LifeBar);
}

fn update_life_bar(
    q_player: Query<(&Life, &MaxLife), With<Player>>,
    mut q_bar: Query<&mut ProgressBar, With<LifeBar>>,
) {
    if let Ok(mut progressbar) = q_bar.single_mut() {
        if let Ok((life, max_life)) = q_player.single() {
            progressbar.max = **max_life;
            progressbar.value = **life;
        }
    }
}

fn update_life_bar_on_death(
    _trigger: Trigger<PlayerDeathEvent>,
    mut q_bar: Query<&mut ProgressBar, With<LifeBar>>,
) {
    if let Ok(mut progressbar) = q_bar.single_mut() {
        progressbar.value = 0.0;
    }
}
