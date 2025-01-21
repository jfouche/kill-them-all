use bevy::prelude::*;

use crate::{components::despawn_all, schedule::GameState};

#[derive(Component)]
#[require(
    Text(|| Text("Kill'em all".into())),
    TextFont(|| TextFont::from_font_size(80.)),
    TextColor(|| TextColor(Color::WHITE))
)]
struct SplashScreenTitle;

#[derive(Component)]
#[require(
    Text(|| Text("Press any key to continue".into())),
    TextFont(|| TextFont::from_font_size(16.)),
    TextColor(|| TextColor(Color::BLACK))
)]
struct SplashScreenMessage;

#[derive(Component)]
#[require(
    Name(|| Name::new("SplashScreen")),
    Node(|| Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    })
)]
struct SplashScreen;

const BACKGROUND_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), spawn_splash_screen)
        .add_systems(OnExit(GameState::Splash), despawn_all::<SplashScreen>)
        .add_systems(Update, goto_main_menu.run_if(in_state(GameState::Splash)));
}

fn spawn_splash_screen(mut commands: Commands) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    commands.spawn(SplashScreen).with_children(|parent| {
        parent.spawn(SplashScreenTitle);
        parent.spawn(SplashScreenMessage);
    });
}

// fn display_continue(mut messages: Query<&mut Text, With<SplashScreenMessage>>) {
//     for mut text in &mut messages {
//         text.sections[0].value = "Press any key to continue".into();
//     }
// }

fn goto_main_menu(
    mut game_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if keys.get_pressed().len() != 0 || mouse.pressed(MouseButton::Left) {
        game_state.set(GameState::Menu);
    }
}
