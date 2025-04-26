use crate::{
    components::despawn_all,
    schedule::{GameState, InGameState},
    ui::{
        button::{SelectedOption, TextButton},
        popup::Popup,
    },
};
use bevy::prelude::*;

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
#[require(
    Popup = Popup::default().with_title("Player died!"),
    Name::new("PlayerDiedMenu")
)]
struct PlayerDiedMenu;

#[derive(Component)]
#[require(TextButton::big("Back to menu"), SelectedOption)]
struct BackToMenu;

fn spawn_player_died_menu(mut commands: Commands) {
    commands.spawn(PlayerDiedMenu).with_children(|popup| {
        popup.spawn(BackToMenu);
    });
}

fn back_to_menu(
    mut q_btn: Query<&Interaction, (Changed<Interaction>, With<BackToMenu>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut in_game_state: ResMut<NextState<InGameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        game_state.set(GameState::Menu);
        in_game_state.set(InGameState::Disabled);
    }
    for interaction in &mut q_btn {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::Menu);
            in_game_state.set(InGameState::Disabled);
        }
    }
}
