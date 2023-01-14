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

use bevy_ecs_ldtk::{prelude::RegisterLdtkObjects, LevelSelection};
use prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Kill'em All".to_string(),
                        width: 1000.0,
                        height: 700.0,
                        position: WindowPosition::At(Vec2::new(250., 0.)),
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(bevy_ui_navigation::DefaultNavigationPlugins)
        // LDtk
        .add_plugin(bevy_ecs_ldtk::LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        // debug plugins
        .add_plugin(debug::DebugPlugin)
        // utils plugins
        .add_plugin(ui::ProgressBarPlugin)
        .add_plugin(utils::blink::BlinkPlugin)
        .add_plugin(utils::invulnerable::InvulnerabilityPlugin)
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
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}

fn load_font(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<Font> = server.load("fonts/FiraSans-Bold.ttf");
    commands.insert_resource(UiFont(handle));
}
