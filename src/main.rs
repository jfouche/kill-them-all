mod camera;
mod components;
mod config;
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

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const APP_TITLE: &str = "Kill'em All";

fn main() {
    let mut app = App::new();

    #[cfg(all(not(target_arch = "wasm32"), feature = "dev"))]
    let window = Window {
        title: APP_TITLE.into(),
        position: WindowPosition::At(IVec2::new(0, 0)),
        resolution: bevy::window::WindowResolution::new(1300., 600.),
        ..Default::default()
    };

    #[cfg(all(not(target_arch = "wasm32"), not(feature = "dev")))]
    let window = Window {
        title: APP_TITLE.into(),
        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
        resizable: false,
        ..Default::default()
    };

    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    let window = Window {
        title: APP_TITLE.into(),
        fit_canvas_to_parent: true,
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
        config::GameConfigPlugin,
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
