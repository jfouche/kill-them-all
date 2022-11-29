use crate::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(switch_game_state)
            .add_system_set(SystemSet::on_enter(GameState::GamePaused).with_system(on_pause))
            .add_system_set(SystemSet::on_exit(GameState::GamePaused).with_system(release_pause));
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

fn on_pause(mut conf: ResMut<RapierConfiguration>) {
    conf.physics_pipeline_active = false;
}

fn release_pause(mut conf: ResMut<RapierConfiguration>) {
    conf.physics_pipeline_active = true;
}
