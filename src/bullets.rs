use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::*;

const BULLET_SPEED: f32 = 20.0;

pub struct BulletOptions {
    pub pos: Vec2,
    pub direction: Vec2,
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
    // events: ActiveEvents,
    bullet: Bullet,
}

impl BulletBundle {
    fn new(options: BulletOptions) -> Self {
        let velocity = options.direction.normalize() * BULLET_SPEED;
        BulletBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(0.2, 0.2)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(options.pos.x, options.pos.y, 0.),
                ..Default::default()
            },
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(0.1, 0.1),
            gravity: GravityScale(0.0),
            constraints: LockedAxes::ROTATION_LOCKED,
            // events: ActiveEvents::CONTACT_FORCE_EVENTS,
            velocity: Velocity::linear(velocity),
            bullet: Bullet,
        }
    }
}

///
///
///
pub fn spawn_bullet_at(
    commands: &mut Commands,
    // materials: &Res<Materials>,
    options: BulletOptions,
) {
    commands.spawn_bundle(BulletBundle::new(options));
}
