use crate::prelude::*;
use std::f32::consts::SQRT_2;

use super::collisions::{GROUP_BONUS, GROUP_BULLET};

const BULLET_SPEED: f32 = 25.0;

pub struct BulletOptions {
    pos: Vec3,
    damage: u16,
    direction: Vect,
    size: Vec2,
}

impl BulletOptions {
    pub fn new(player_pos: Vec3, damage: u16, player_size: Vec2, target: Vec3) -> Self {
        let dir = target - player_pos;
        BulletOptions {
            pos: player_pos,
            damage,
            direction: Vect::new(dir.x, dir.y),
            size: player_size,
        }
    }
}

///
///  Retrieve the pos of the bullet, according to an Ellipse around the player
///
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
    let velocity = options.direction.normalize() * BULLET_SPEED;
    let pos = ellipse_pos(&options);
    let size = 0.3;
    commands
        .spawn(Bullet)
        .insert(Name::new("Bullet"))
        .insert(Damage(options.damage))
        // Sprite
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(size, size)),
                ..Default::default()
            },
            transform: Transform::from_translation(pos),
            ..Default::default()
        })
        // Rapier
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(size / 2., size / 2.))
        .insert(CollisionGroups::new(
            GROUP_BULLET,
            Group::ALL & !GROUP_BONUS,
        ))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::linear(velocity));
}
