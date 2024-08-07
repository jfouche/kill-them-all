use bevy::prelude::*;

/// Represent the Game state
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    InGame,
}

/// Represent the state while in game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InGameState {
    #[default]
    Disabled,
    Running,
    Pause,
    // PlayerEndedLevel,
    // PlayerDied,
    // ShowPopup,
    // LoadLevel,
    LevelUp,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
pub enum InGameSet {
    UserInput,
    EntityUpdate,
    CollisionDetection,
    DespawnEntities,
}

pub fn schedule_plugin(app: &mut App) {
    app.init_state::<GameState>()
        .init_state::<InGameState>()
        .configure_sets(
            Update,
            (
                InGameSet::DespawnEntities,
                // apply_deffer will be added here
                InGameSet::UserInput,
                InGameSet::EntityUpdate,
                InGameSet::CollisionDetection,
            )
                .chain()
                .run_if(game_is_running),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(InGameSet::DespawnEntities)
                .before(InGameSet::UserInput),
        )
        .add_systems(OnExit(GameState::InGame), end_game);
}

fn game_is_running(
    game_state: Res<State<GameState>>,
    in_game_state: Res<State<InGameState>>,
) -> bool {
    *game_state == GameState::InGame && *in_game_state == InGameState::Running
}

fn end_game(mut in_game_state: ResMut<NextState<InGameState>>) {
    in_game_state.set(InGameState::Disabled);
}
