use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::{prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;

mod components;
mod player;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Platformer!".to_string(),
            width: 640.0,
            height: 400.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // debug plugins
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        // Game plugins
        .add_plugin(player::PlayerPlugin)
        // startup
        .add_startup_system(startup)
        // systems
        .run();
}

fn startup(mut commands: Commands) {
    let far = 1000.0;
    let mut camera = Camera2dBundle::new_with_far(far);
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(30.0);
    commands.spawn_bundle(camera);
    
    commands.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)));
}