use crate::{
    components::despawn_all,
    schedule::{GameState, InGameState},
    ui::{
        button::TextButton,
        popup::{Popup, PopupTitle},
    },
};
use bevy::{ecs::spawn::SpawnWith, prelude::*};

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
                back_to_menu.run_if(in_state(InGameState::PlayerDied)),
            );
    }
}

#[derive(Component)]
struct PlayerDiedMenu;

fn player_died_menu() -> impl Bundle {
    (
        PlayerDiedMenu,
        Name::new("PlayerDiedMenu"),
        Popup,
        Children::spawn((
            Spawn(PopupTitle::bundle("Player died!")),
            SpawnWith(|menu: &mut ChildSpawner| {
                menu.spawn(TextButton::big("Back to menu")).observe(
                    |_t: Trigger<Pointer<Click>>,
                     mut game_state: ResMut<NextState<GameState>>,
                     mut in_game_state: ResMut<NextState<InGameState>>| {
                        game_state.set(GameState::Menu);
                        in_game_state.set(InGameState::Disabled);
                    },
                );
            }),
        )),
    )
}

fn spawn_player_died_menu(mut commands: Commands) {
    commands.spawn(player_died_menu());
}

fn back_to_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        game_state.set(GameState::Menu);
        in_game_state.set(InGameState::Disabled);
    }
}
