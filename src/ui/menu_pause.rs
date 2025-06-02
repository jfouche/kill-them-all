use crate::{
    components::despawn_all,
    in_game::back_to_game,
    schedule::{GameState, InGameState},
    theme::widget::button,
    ui::popup::{Popup, PopupTitle},
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
                menu.spawn(button("Back to game", on_back_to_game));
                menu.spawn(button("Quit game", on_quit_game));
            }),
        )),
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
