use std::ops::Mul;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

const PLAYER_SIZE: Vec2 = Vec2::new(1.0, 1.0);

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    player: Player,
    body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    constraints: LockedAxes,
    gravity: GravityScale,
    // events: ActiveEvents
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.3, 0.4, 0.8),
                    custom_size: Some(PLAYER_SIZE),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., 0., 10.),
                ..Default::default()
            },
            player: Player { speed: 8. },
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(PLAYER_SIZE.x/2., PLAYER_SIZE.y/2.),
            gravity: GravityScale(0.0),
            constraints: LockedAxes::ROTATION_LOCKED,
            //     events: ActiveEvents::COLLISION_EVENTS,
            velocity: Velocity::default(),
        }
    }
}

///
///  spawn player system
///
fn spawn_player(mut commands: Commands, mut _materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(PlayerBundle::default())
        .insert(Name::new("Player"));
}

///
/// Manage the keyboard to move the player
///
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut velocity) in players.iter_mut() {
        let mut linvel = Vec2::default();
        if keyboard_input.any_pressed([KeyCode::Left, KeyCode::Numpad4]) {
            linvel.x = -1.0;
        }
        if keyboard_input.any_pressed([KeyCode::Right, KeyCode::Numpad6]) {
            linvel.x = 1.0;
        }
        if keyboard_input.any_pressed([KeyCode::Up, KeyCode::Numpad8]) {
            linvel.y = 1.0;
        }
        if keyboard_input.any_pressed([KeyCode::Down, KeyCode::Numpad2]) {
            linvel.y = -1.0;
        }
            velocity.linvel = linvel.normalize_or_zero().mul(player.speed);
    }
}

///
/// 
/// 
fn player_touched_by_monster() {

}