use crate::components::*;
use crate::in_game::run_game;
use crate::schedule::*;
use crate::ui::*;
use bevy::prelude::*;

pub struct RoundEndMenuPlugin;

impl Plugin for RoundEndMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::RoundEnd), spawn_round_end_menu)
            .add_systems(OnExit(InGameState::RoundEnd), despawn_all::<RoundEndMenu>)
            .add_systems(Update, back_to_game.run_if(in_state(InGameState::RoundEnd)));
    }
}

#[derive(Component)]
struct RoundEndMenu;

#[derive(Component)]
struct BackToMenu;

fn spawn_round_end_menu(mut commands: Commands) {
    commands
        .spawn_popup("End of round", RoundEndMenu)
        .with_children(|popup| {
            popup.spawn_button("Back to game", BackToMenu);
        });
}

pub fn back_to_game(state: ResMut<NextState<InGameState>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        run_game(state);
    }
}
