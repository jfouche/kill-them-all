mod bullets;
mod collisions;
mod components;
mod events;
mod monster;
mod player;
mod prelude;
mod resources;
mod top_menu;
mod ui;
mod world;

use bevy::render::camera::ScalingMode;
use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Platformer!".to_string(),
                width: 1024.0,
                height: 730.0,
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // debug plugins
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        // utils plugins
        .add_plugin(ui::ProgressBarPlugin)
        // Game plugins
        .add_plugin(world::WorldPlugin)
        .add_plugin(top_menu::TopMenuPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(monster::MonsterPlugin)
        .add_plugin(collisions::CollisionsPlugin)
        // resources
        .init_resource::<ScoreResource>()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        // startup
        .add_startup_system_to_stage(StartupStage::PreStartup, load_font)
        .add_startup_system(init_camera)
        // systems
        // RUN
        .run();
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
