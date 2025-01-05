use crate::components::*;
use crate::in_game::back_to_game;
use crate::schedule::*;
use crate::ui::*;
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
#[require(
    Name(|| Name::new("PauseMenu")),
    Popup(|| Popup::default().with_title("Pause"))
)]
struct PauseMenu;

#[derive(Component)]
#[require(
    MyButton(|| MyButton::new("Back to game")),
    MenuButtonAction(|| MenuButtonAction::BackToGame),
)]
pub struct ButtonBackToGame;

#[derive(Component)]
#[require(
    MyButton(|| MyButton::new("Quit game")),
    MenuButtonAction(|| MenuButtonAction::QuitGame),
)]
pub struct ButtonQuitGame;

// All actions that can be triggered from a button click
#[derive(Component, Clone, Copy, PartialEq)]
enum MenuButtonAction {
    BackToGame,
    QuitGame,
}

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn(PauseMenu).with_children(|menu| {
        menu.spawn(ButtonBackToGame).observe(menu_action);
        menu.spawn(ButtonQuitGame).observe(menu_action);
    });
}

fn menu_action(
    trigger: Trigger<Pointer<Click>>,
    actions: Query<&MenuButtonAction>,
    mut in_game_state: ResMut<NextState<InGameState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(action) = actions.get(trigger.entity()) {
        match action {
            MenuButtonAction::BackToGame => {
                in_game_state.set(InGameState::Running);
            }
            MenuButtonAction::QuitGame => {
                game_state.set(GameState::Menu);
            }
        }
    }
}
