use std::f32::consts::SQRT_2;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::*;

const BULLET_SPEED: f32 = 20.0;

pub struct BulletOptions {
    pos: Vec3,
    direction: Vect,
    size: Vec2,
}

impl BulletOptions {
    pub fn new(player_pos: Vec3, player_size: Vec2, target: Vec3) -> Self {
        let dir = target - player_pos;
        BulletOptions {
            pos: player_pos,
            direction: Vect::new(dir.x, dir.y),
            size: player_size,
        }
    }
}

#[derive(Bundle)]
struct BulletBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
    gravity: GravityScale,
    velocity: Velocity,
    constraints: LockedAxes,
    events: ActiveEvents,
    bullet: Bullet,
}

impl BulletBundle {
    fn new(options: BulletOptions) -> Self {
        let velocity = options.direction.normalize() * BULLET_SPEED;
        let pos = ellipse_pos(&options);
        let size = 0.3;
        BulletBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(size, size)),
                    ..Default::default()
                },
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(size / 2., size / 2.),
            gravity: GravityScale(0.0),
            constraints: LockedAxes::ROTATION_LOCKED,
            events: ActiveEvents::COLLISION_EVENTS,
            velocity: Velocity::linear(velocity),
            bullet: Bullet,
        }
    }
}

fn ellipse_pos(options: &BulletOptions) -> Vec3 {
    let angle = Vec2::X.angle_between(options.direction);
    let x = angle.cos() * SQRT_2 * options.size.x / 2.0 + options.pos.x;
    let y = angle.sin() * SQRT_2 * options.size.y / 2.0 + options.pos.y;
    Vec3::new(x, y, 20.)
}

///
///
///
pub fn spawn_bullet_at(
    commands: &mut Commands,
    // materials: &Res<Materials>,
    options: BulletOptions,
) {
    commands
        .spawn(BulletBundle::new(options))
        .insert(Name::new("Bullet"));
}
