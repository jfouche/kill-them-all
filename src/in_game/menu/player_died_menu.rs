use crate::components::*;
use crate::schedule::*;
use crate::ui::{spawn_button, spawn_popup};
use bevy::prelude::*;

pub struct PlayerDiedPlugin;

impl Plugin for PlayerDiedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::PlayerDied), spawn_player_died_menu)
            .add_systems(
                OnExit(InGameState::PlayerDied),
                despawn_all::<PlayerDiedMenu>,
            )
            .add_systems(
                Update,
                back_to_menu.run_if(in_state(InGameState::PlayerDied)),
            );
    }
}

#[derive(Component)]
struct PlayerDiedMenu;

#[derive(Component)]
struct BackToMenu;

fn spawn_player_died_menu(commands: Commands) {
    spawn_popup(commands, "Player died!", PlayerDiedMenu, |popup| {
        spawn_button(popup, "Back to menu", BackToMenu);
    });
}

fn back_to_menu(
    mut q_btn: Query<&Interaction, (Changed<Interaction>, With<BackToMenu>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    for interaction in &mut q_btn {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::Menu);
            in_game_state.set(InGameState::Disabled);
        }
    }
}
