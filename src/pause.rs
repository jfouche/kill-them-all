use crate::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(switch_game_state);
    }
}

fn switch_game_state(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.current() {
            GameState::InGame => state.set(GameState::GamePaused).unwrap(),
            GameState::GamePaused => state.set(GameState::InGame).unwrap(),
        }
    }
}
