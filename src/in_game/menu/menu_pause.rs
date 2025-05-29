use crate::{
    components::despawn_all,
    in_game::back_to_game,
    schedule::{GameState, InGameState},
    ui::{
        button::TextButton,
        popup::{Popup, PopupTitle},
    },
};
use bevy::{ecs::spawn::SpawnWith, prelude::*};

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
        Popup,
        Children::spawn((
            Spawn(PopupTitle::bundle("Pause")),
            SpawnWith(|menu: &mut ChildSpawner| {
                menu.spawn(TextButton::big("Back to game")).observe(
                    |_t: Trigger<Pointer<Click>>, mut state: ResMut<NextState<InGameState>>| {
                        state.set(InGameState::Running);
                    },
                );
                menu.spawn(TextButton::big("Quit game")).observe(
                    |_t: Trigger<Pointer<Click>>, mut state: ResMut<NextState<GameState>>| {
                        state.set(GameState::Menu);
                    },
                );
            }),
        )),
    )
}

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn(pause_menu());
}
