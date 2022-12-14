mod components;
mod debug;
mod events;
mod in_game;
mod level_up_menu;
mod pause_menu;
mod prelude;
mod resources;
mod top_menu;
mod ui;
mod utils;

use bevy::render::camera::ScalingMode;
use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Kill'em All".to_string(),
                width: 1024.0,
                height: 730.0,
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(bevy_ui_navigation::DefaultNavigationPlugins)
        // debug plugins
        .add_plugin(debug::DebugPlugin)
        // utils plugins
        .add_plugin(ui::ProgressBarPlugin)
        .add_plugin(utils::BlinkPlugin)
        .add_plugin(utils::InvulnerabilityPlugin)
        // Game plugins
        .add_plugin(top_menu::TopMenuPlugin)
        .add_plugins(in_game::InGamePluginsGroup)
        .add_plugin(pause_menu::PausePlugin)
        .add_plugin(level_up_menu::LevelUpMenuPlugin)
        // States
        .add_state(GameState::InGame)
        // resources
        .init_resource::<ScoreResource>()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .init_resource::<GameTextures>()
        // Events
        .add_event::<PlayerHitEvent>()
        .add_event::<PlayerDeathEvent>()
        .add_event::<MonsterHitEvent>()
        .add_event::<MonsterDeathEvent>()
        .add_event::<LevelUpEvent>()
        // startup
        .add_startup_system_to_stage(StartupStage::PreStartup, load_font)
        .add_startup_system(init_rapier)
        .add_startup_system(init_camera)
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
