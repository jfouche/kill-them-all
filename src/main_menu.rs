use crate::{components::despawn_all, schedule::*, ui::*};
use bevy::{app::AppExit, color::palettes::css::GRAY, prelude::*};

pub fn main_menu_plugin(app: &mut App) {
    app.init_resource::<MainMenuButtonNav>()
        .add_systems(OnEnter(GameState::Menu), (set_background, spawn_menu))
        .add_systems(OnExit(GameState::Menu), despawn_all::<MainMenu>)
        .add_systems(
            Update,
            (button_keyboard_nav::<MainMenuButtonNav>, menu_action)
                .chain()
                .run_if(in_state(GameState::Menu)),
        );
}

#[derive(Component)]
#[require(
    Popup(|| Popup::default().with_title("Kill'em all")),
    Name(|| Name::new("MainMenu"))
)]
struct MainMenu;

#[derive(Component)]
#[require(
    MyButton(|| MyButton::new("New game")),
    MenuButtonAction(|| MenuButtonAction::PlayGame),
)]
pub struct ButtonNewGame;

#[derive(Component)]
#[require(
    MyButton(|| MyButton::new("Exit")),
    MenuButtonAction(|| MenuButtonAction::ExitApplication),
)]
pub struct ButtonExit;

// All actions that can be triggered from a button click
#[derive(Component, Clone, Copy, PartialEq)]
enum MenuButtonAction {
    PlayGame,
    ExitApplication,
}

#[derive(Resource, Default)]
struct MainMenuButtonNav(Vec<Entity>);

impl std::ops::Deref for MainMenuButtonNav {
    type Target = [Entity];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn set_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(GRAY.into()));
}

fn spawn_menu(mut commands: Commands) {
    let new_game_btn = commands.spawn((ButtonNewGame, SelectedOption)).id();
    let exit_btn = commands.spawn(ButtonExit).id();

    let menu_nav = MainMenuButtonNav(vec![new_game_btn, exit_btn]);

    commands.spawn(MainMenu).add_children(&menu_nav);

    commands.insert_resource(menu_nav);
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
