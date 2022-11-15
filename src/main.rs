use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use resources::ScoreResource;

mod bullets;
mod collisions;
mod components;
mod monster;
mod player;
mod resources;
mod top_menu;
mod world;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Platformer!".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // debug plugins
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        // Game plugins
        .add_plugin(world::WorldPlugin)
        .add_plugin(top_menu::TopMenuPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(monster::MonsterPlugin)
        .add_plugin(collisions::CollisionsPlugin)
        // resources
        .init_resource::<ScoreResource>()
        // startup
        .add_startup_system(startup)
        // systems
        // RUN
        .run();
}

fn startup(mut commands: Commands) {
    let far = 1000.0;
    let mut camera = Camera2dBundle::new_with_far(far);
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(40.0);
    commands.spawn_bundle(camera);

    commands.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)));
}
