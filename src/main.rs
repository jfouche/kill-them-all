mod components;
mod cursor;
mod in_game;
mod main_menu;
mod resources;
mod schedule;
mod splash;
mod ui;
mod utils;

#[cfg(feature = "debug")]
mod debug;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use components::*;
use resources::{ScoreResource, UiFont};

fn main() {
    let mut app = App::new();
    app.add_plugins(
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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.))
        // utils plugins
        .add_plugins((
            ui::UiPlugins,
            utils::blink::BlinkPlugin,
            utils::invulnerable::InvulnerabilityPlugin,
        ))
        // Game plugins
        .add_plugins((
            schedule::schedule_plugin,
            splash::splash_plugin,
            main_menu::main_menu_plugin,
            in_game::InGamePluginsGroup,
        ))
        // resources
        .init_resource::<ScoreResource>()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        // Events
        .add_event::<LevelUpEvent>()
        // startup
        .add_systems(PreStartup, load_font)
        .add_systems(Startup, (init_rapier, init_camera))
        // systems
        ;

    #[cfg(feature = "debug")]
    app.add_plugins(debug::DebugPlugin);

    // RUN
    app.run();
}

fn init_rapier(mut conf: ResMut<RapierConfiguration>) {
    conf.gravity = Vect::ZERO;
}

fn init_camera(mut commands: Commands) {
    // let far = 1000.0;
    // let mut camera = Camera2dBundle::new_with_far(far);
    // camera.projection.scaling_mode = ScalingMode::FixedHorizontal(40.0);
    commands.spawn(Camera2dBundle::default());
}

fn load_font(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<Font> = server.load("fonts/FiraSans-Bold.ttf");
    commands.insert_resource(UiFont(handle));
}
