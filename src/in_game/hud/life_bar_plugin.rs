use super::Hud;
use crate::{
    components::*,
    in_game::{GameRunningSet, GameState},
    ui::{ProgressBar, ProgressBarColor},
};
use bevy::{color::palettes::css::RED, prelude::*};

pub struct LifeBarPlugin;

impl Plugin for LifeBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_life_bar)
            .add_systems(
                Update,
                (update_life_bar, update_life_bar_on_death).in_set(GameRunningSet::EntityUpdate),
            );
    }
}

#[derive(Component)]
#[require(
    Hud,
    Name(|| Name::new("HUD - LifeBar")),
    Node(|| Node {
        position_type: PositionType::Absolute,
        left: Val::Px(50.),
        top: Val::Px(20.),
        width: Val::Px(300.),
        height: Val::Px(20.),
        border: UiRect::all(Val::Px(2.)),
        ..Default::default()
    }),
    BackgroundColor(|| BackgroundColor(Color::BLACK)),
    BorderColor(|| BorderColor(Color::BLACK)),
    ProgressBar,
    ProgressBarColor(|| ProgressBarColor(RED.into()))
)]
struct LifeBar;

fn spawn_life_bar(mut commands: Commands) {
    commands.spawn(LifeBar);
}

fn update_life_bar(
    q_player: Query<(&Life, &MaxLife), With<Player>>,
    mut q_bar: Query<&mut ProgressBar, With<LifeBar>>,
) {
    if let Ok(mut progressbar) = q_bar.get_single_mut() {
        if let Ok((life, max_life)) = q_player.get_single() {
            progressbar.max = **max_life;
            progressbar.value = **life;
        }
    }
}

fn update_life_bar_on_death(
    mut player_death_events: EventReader<PlayerDeathEvent>,
    mut q_bar: Query<&mut ProgressBar, With<LifeBar>>,
) {
    if let Ok(mut progressbar) = q_bar.get_single_mut() {
        for _ in player_death_events.read() {
            progressbar.value = 0.0;
        }
    }
}
