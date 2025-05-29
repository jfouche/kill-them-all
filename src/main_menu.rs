use crate::{
    components::despawn_all,
    schedule::GameState,
    ui::{
        button::TextButton,
        popup::{Popup, PopupTitle},
    },
};
use bevy::{app::AppExit, color::palettes::css::GRAY, ecs::spawn::SpawnWith, prelude::*};

pub fn main_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), (set_background, spawn_menu))
        .add_systems(OnExit(GameState::Menu), despawn_all::<MainMenu>);
}

#[derive(Component)]
struct MainMenu;

fn main_menu() -> impl Bundle {
    (
        MainMenu,
        Name::new("MainMenu"),
        Popup,
        Children::spawn((
            Spawn(PopupTitle::bundle("Kill'em all")),
            SpawnWith(|menu: &mut ChildSpawner| {
                menu.spawn(TextButton::big("New game")).observe(
                    |_t: Trigger<Pointer<Click>>,
                     mut next_game_state: ResMut<NextState<GameState>>| {
                        next_game_state.set(GameState::InGame);
                    },
                );
                menu.spawn(TextButton::big("Exit")).observe(
                    |_t: Trigger<Pointer<Click>>, mut app_exit_events: EventWriter<AppExit>| {
                        app_exit_events.write(AppExit::Success);
                    },
                );
            }),
        )),
    )
}

fn set_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(GRAY.into()));
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn(main_menu());
}
