use crate::components::*;
use crate::schedule::*;
use crate::utils::cursor::*;
use bevy::prelude::*;

pub fn splash_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Splash),
        (spawn_splash_screen, ungrab_cursor),
    )
    .add_systems(OnExit(GameState::Splash), despawn_all::<SplashScreen>)
    .add_systems(Update, goto_main_menu.run_if(in_state(GameState::Splash)));
}

#[derive(Component)]
struct SplashScreen;

#[derive(Component)]
struct SplashScreenMessage;

const BACKGROUND_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);

fn spawn_splash_screen(mut commands: Commands) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    commands
        .spawn((
            SplashScreen,
            Name::new("SplashScreen"),
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text("Kill'em all".into()),
                TextFont::from_font_size(80.),
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                SplashScreenMessage,
                Text("Press any key to continue".into()),
                TextFont::from_font_size(16.),
                TextColor(Color::BLACK),
            ));
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
    if keys.get_pressed().len() != 0 {
        game_state.set(GameState::Menu);
    }
    if mouse.pressed(MouseButton::Left) {
        game_state.set(GameState::Menu);
    }
}
