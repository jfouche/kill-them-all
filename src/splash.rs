use crate::{asset_tracking::ResourceHandles, components::despawn_all, schedule::GameState};
use bevy::prelude::*;

#[derive(Component)]
#[require(
    Text("Kill'em all".into()),
    TextFont::from_font_size(80.),
    TextColor(Color::WHITE)
)]
struct SplashScreenTitle;

#[derive(Component)]
#[require(
    Text("Press any key to continue".into()),
    TextFont::from_font_size(16.),
    TextColor(Color::BLACK)
)]
struct SplashScreenMessage;

#[derive(Component)]
#[require(
    Name::new("SplashScreen"),
    Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }
)]
struct SplashScreen;

impl SplashScreen {
    pub fn bundle() -> impl Bundle {
        (
            SplashScreen,
            children![SplashScreenTitle, SplashScreenMessage],
        )
    }
}

const BACKGROUND_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), spawn_splash_screen)
        .add_systems(OnExit(GameState::Splash), despawn_all::<SplashScreen>)
        .add_systems(Update, goto_main_menu.run_if(in_state(GameState::Splash)));
}

fn spawn_splash_screen(mut commands: Commands) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    commands.spawn(SplashScreen::bundle());
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
    resources: Res<ResourceHandles>,
) {
    if resources.is_all_done() {
        if keys.get_pressed().len() != 0 || mouse.pressed(MouseButton::Left) {
            game_state.set(GameState::Menu);
        }
    }
}
