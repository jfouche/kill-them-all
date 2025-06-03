use crate::{
    components::despawn_all,
    schedule::{GameState, InGameState},
    theme::widget,
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub struct PlayerDiedMenuPlugin;

impl Plugin for PlayerDiedMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::PlayerDied), spawn_player_died_menu)
            .add_systems(
                OnExit(InGameState::PlayerDied),
                despawn_all::<PlayerDiedMenu>,
            )
            .add_systems(
                Update,
                back_to_menu.run_if(
                    in_state(InGameState::PlayerDied).and(input_just_pressed(KeyCode::Enter)),
                ),
            );
    }
}

#[derive(Component)]
struct PlayerDiedMenu;

fn player_died_menu() -> impl Bundle {
    (
        PlayerDiedMenu,
        Name::new("PlayerDiedMenu"),
        widget::popup(),
        children![
            widget::popup_title("Player died!"),
            widget::button("Back to menu", on_back_to_menu)
        ],
    )
}

fn spawn_player_died_menu(mut commands: Commands) {
    commands.spawn(player_died_menu());
}

fn back_to_menu(
    mut game_state: ResMut<NextState<GameState>>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    game_state.set(GameState::Menu);
    in_game_state.set(InGameState::Disabled);
}

fn on_back_to_menu(
    _trigger: Trigger<Pointer<Click>>,
    game_state: ResMut<NextState<GameState>>,
    in_game_state: ResMut<NextState<InGameState>>,
) {
    back_to_menu(game_state, in_game_state);
}
