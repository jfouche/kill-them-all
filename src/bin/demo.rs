use avian2d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(16.),
            PhysicsDebugPlugin::default(),
        ))
        .insert_resource(Gravity(Vec2::ZERO))
        .add_systems(Startup, setup)
        .add_observer(move_player)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player,
        RigidBody::Kinematic,
        Collider::rectangle(8., 8.),
        children![Collider::circle(16.)],
    ));
}

fn move_player(
    trigger: Trigger<Pointer<Pressed>>,
    mut players: Query<(&Transform, &mut LinearVelocity), With<Player>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    if let Some(world_pos) = world_position(cameras, trigger.pointer_location.position) {
        if let Ok((player_transform, mut velocity)) = players.single_mut() {
            let player_pos = player_transform.translation.xy();
            let dir = world_pos - player_pos;
            *velocity = LinearVelocity(dir);
        }
    }
}

fn world_position(cameras: Query<(&Camera, &GlobalTransform)>, pos: Vec2) -> Option<Vec2> {
    cameras
        .single()
        .ok()
        .and_then(|(camera, transform)| camera.viewport_to_world_2d(transform, pos).ok())
}
