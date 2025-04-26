use crate::{components::player::Player, schedule::GameRunningSet};
use bevy::prelude::*;

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera).add_systems(
        Update,
        (camera_follow_player).in_set(GameRunningSet::EntityUpdate),
    );
}

#[derive(Component)]
#[require(
    Name::new("MainCamera"),
    Camera2d,
    Camera {
        hdr: true,
        ..Default::default()
    },
    Projection::custom(OrthographicProjection{
        scale: 0.5,
        ..OrthographicProjection::default_2d()
    }),
)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((MainCamera, IsDefaultUiCamera));
}

fn camera_follow_player(
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let Ok(mut camera) = camera.single_mut() else {
        return;
    };
    if let Ok(player) = player.single() {
        camera.translation = player.translation;
    }
}
