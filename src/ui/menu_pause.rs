use crate::{
    components::despawn_all,
    in_game::back_to_game,
    schedule::{GameState, InGameState},
    theme::widget::{self, button},
};
use bevy::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::Pause), spawn_pause_menu)
            .add_systems(OnExit(InGameState::Pause), despawn_all::<PauseMenu>)
            .add_systems(Update, back_to_game.run_if(in_state(InGameState::Pause)));
    }
}

#[derive(Component)]
struct PauseMenu;

fn pause_menu() -> impl Bundle {
    (
        PauseMenu,
        Name::new("PauseMenu"),
        widget::popup(),
        children![
            widget::popup_title("Pause"),
            button("Back to game", on_back_to_game),
            button("Quit game", on_quit_game)
        ],
    )
}

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn(pause_menu());
}

fn on_back_to_game(_trigger: Trigger<Pointer<Click>>, mut state: ResMut<NextState<InGameState>>) {
    state.set(InGameState::Running);
}

fn on_quit_game(_trigger: Trigger<Pointer<Click>>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Menu);
}
