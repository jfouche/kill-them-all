mod components;
mod debug;
mod hud;
mod in_game;
mod level_up_menu;
mod pause_menu;
mod prelude;
mod resources;
mod ui;
mod utils;

use bevy::render::camera::ScalingMode;
use prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Kill'em All".to_string(),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // debug plugins
        .add_plugins(debug::DebugPlugin)
        // utils plugins
        .add_plugins((
            ui::UiPlugins,
            utils::blink::BlinkPlugin,
            utils::invulnerable::InvulnerabilityPlugin,
        ))
        // Game plugins
        .add_plugins((
            hud::TopMenuPlugin,
            in_game::InGamePluginsGroup,
            pause_menu::PausePlugin,
            level_up_menu::LevelUpMenuPlugin,
        ))
        // States
        .init_state::<GameState>()
        // resources
        .init_resource::<ScoreResource>()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        // Events
        .add_event::<LevelUpEvent>()
        // startup
        .add_systems(PreStartup, load_font)
        .add_systems(Startup, (init_rapier, init_camera))
        // systems
        // RUN
        .run();
}

fn init_rapier(mut conf: ResMut<RapierConfiguration>) {
    conf.gravity = Vect::ZERO;
}

fn init_camera(mut commands: Commands) {
    let far = 1000.0;
    let mut camera = Camera2dBundle::new_with_far(far);
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(40.0);
    commands.spawn(camera);
}

fn load_font(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<Font> = server.load("fonts/FiraSans-Bold.ttf");
    commands.insert_resource(UiFont(handle));
}
