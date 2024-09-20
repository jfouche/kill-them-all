mod camera;
mod components;
mod cursor;
mod in_game;
mod main_menu;
mod schedule;
mod splash;
mod ui;
mod utils;

#[cfg(feature = "debug")]
mod debug;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
            utils::despawn_after::despawn_after_plugin
        ))
        // Game plugins
        .add_plugins((
            schedule::schedule_plugin,
            camera::camera_plugin,
            splash::splash_plugin,
            main_menu::main_menu_plugin,
            in_game::InGamePluginsGroup,
        ))
        // resources
        .init_resource::<components::Score>() // TODO: Move to plugin
        // startup
        .add_systems(Startup, init_rapier)
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
