use crate::{
    components::despawn_all,
    schedule::GameState,
    ui::{
        button::TextButton,
        popup::{Popup, PopupTitle},
    },
};
use bevy::{app::AppExit, color::palettes::css::GRAY, prelude::*};

pub fn main_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), (set_background, spawn_menu))
        .add_systems(OnExit(GameState::Menu), despawn_all::<MainMenu>)
        .add_systems(Update, menu_action.run_if(in_state(GameState::Menu)));
}

#[derive(Component)]
struct MainMenu;

// All actions that can be triggered from a button click
#[derive(Component, Clone, Copy, PartialEq)]
enum MenuButtonAction {
    PlayGame,
    ExitApplication,
}

fn set_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(GRAY.into()));
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        MainMenu,
        Name::new("MainMenu"),
        Popup,
        children![
            PopupTitle::bundle("Kill'em all"),
            (TextButton::big("New game"), MenuButtonAction::PlayGame),
            (TextButton::big("Exit"), MenuButtonAction::ExitApplication)
        ],
    ));
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::ExitApplication => {
                    app_exit_events.write(AppExit::Success);
                }
                MenuButtonAction::PlayGame => {
                    next_game_state.set(GameState::InGame);
                }
            }
        }
    }
}
