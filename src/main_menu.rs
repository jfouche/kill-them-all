use crate::{components::despawn_all, schedule::*, ui::*};
use bevy::{app::AppExit, color::palettes::css::GRAY, prelude::*};

pub fn main_menu_plugin(app: &mut App) {
    app.init_resource::<MainMenuButtonNav>()
        .add_systems(OnEnter(GameState::Menu), (set_background, spawn_menu))
        .add_systems(OnExit(GameState::Menu), despawn_all::<MainMenu>)
        .add_systems(
            Update,
            (
                button_keyboard_nav::<MenuButtonAction, MainMenuButtonNav>,
                menu_action,
            )
                .chain()
                .run_if(in_state(GameState::Menu)),
        );
}

#[derive(Component)]
struct MainMenu;

// All actions that can be triggered from a button click
#[derive(Component, Clone, Copy, PartialEq)]
enum MenuButtonAction {
    PlayGame,
    // Settings,
    // SettingsSound,
    // SettingsDisplay,
    // BackToMainMenu,
    // BackToSettings,
    ExitApplication,
    // QuitGame,
}

#[derive(Resource)]
struct MainMenuButtonNav([MenuButtonAction; 2]);

impl Default for MainMenuButtonNav {
    fn default() -> Self {
        MainMenuButtonNav([
            MenuButtonAction::PlayGame,
            MenuButtonAction::ExitApplication,
        ])
    }
}

impl std::ops::Deref for MainMenuButtonNav {
    type Target = [MenuButtonAction];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn set_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(GRAY.into()));
}

fn spawn_menu(mut commands: Commands) {
    commands
        .spawn_popup("Kill'em all", MainMenu)
        .with_children(|menu| {
            menu.spawn_button("New game", (MenuButtonAction::PlayGame, SelectedOption));
            menu.spawn_button("Exit", MenuButtonAction::ExitApplication);
        });
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
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::PlayGame => {
                    next_game_state.set(GameState::InGame);
                }
            }
        }
    }
}
