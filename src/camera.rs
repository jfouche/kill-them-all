use crate::{components::*, schedule::GameRunningSet};
use bevy::prelude::*;

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera).add_systems(
        Update,
        (camera_follow_player).in_set(GameRunningSet::EntityUpdate),
    );
}

// const ASPECT_RATIO: f32 = 16. / 9.;
const CAM_LERP_FACTOR: f32 = 2.;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Camera {
            hdr: true,
            ..Default::default()
        },
        OrthographicProjection {
            scale: 2.,
            ..OrthographicProjection::default_2d()
        },
        IsDefaultUiCamera,
    ));
}

fn camera_follow_player(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };
    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using interpolation between
    // the camera position and the player position on the x and y axes.
    // Here we use the in-game time, to get the elapsed time (in seconds)
    // since the previous update. This avoids jittery movement when tracking
    // the player.
    camera.translation = camera
        .translation
        .lerp(direction, time.delta_secs() * CAM_LERP_FACTOR);
}
