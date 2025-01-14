mod camera;
mod components;
mod in_game;
mod main_menu;
mod schedule;
mod splash;
mod ui;
mod utils;

#[cfg(test)]
mod test;

#[cfg(feature = "dev")]
mod debug;

use bevy::{prelude::*, window::*};
use bevy_rapier2d::prelude::*;

const APP_TITLE: &str = "Kill'em All";

fn main() {
    let mut app = App::new();

    #[cfg(feature = "dev")]
    let window = Window {
        title: APP_TITLE.into(),
        position: WindowPosition::At(IVec2::new(0, 0)),
        resolution: WindowResolution::new(1400., 600.),
        ..Default::default()
    };

    #[cfg(not(feature = "dev"))]
    let window = Window {
        title: APP_TITLE.into(),
        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
        ..Default::default()
    };

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(window),
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
        utils::despawn_after::despawn_after_plugin,
    ))
    // Game plugins
    .add_plugins((
        schedule::schedule_plugin,
        camera::camera_plugin,
        splash::splash_plugin,
        main_menu::main_menu_plugin,
        in_game::InGamePluginsGroup,
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(debug::DebugPlugin);

    // RUN
    app.run();
}
