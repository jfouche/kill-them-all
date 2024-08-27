use super::*;
use bevy::{color::palettes::css::YELLOW, prelude::*};
use bevy_rapier2d::prelude::*;
use std::f32::consts::SQRT_2;

pub fn gun() -> Weapon {
    Weapon::new(WeaponType::Gun, 1., 1, 2)
}

#[derive(Component)]
pub struct Bullet;

#[derive(Bundle)]
pub struct BulletBundle {
    tag: Bullet,
    name: Name,
    damage: Damage,
    pierce: PierceChance,
    lifetime: LifeTime,
    sprite: SpriteBundle,
    body: RigidBody,
    velocity: Velocity,
    collider: Collider,
    sensor: Sensor,
    collision_groups: CollisionGroups,
    locked_axes: LockedAxes,
    active_events: ActiveEvents,
}

impl Default for BulletBundle {
    fn default() -> Self {
        BulletBundle {
            tag: Bullet,
            name: Name::new("Bullet"),
            damage: Damage::default(),
            pierce: PierceChance::default(),
            lifetime: LifeTime::new(3.),
            sprite: SpriteBundle::default(),
            body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            collider: Collider::default(),
            sensor: Sensor,
            collision_groups: CollisionGroups::new(GROUP_BULLET, Group::ALL & !GROUP_BONUS),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

impl BulletBundle {
    pub fn new(options: BulletOptions) -> Self {
        const BULLET_SPEED: f32 = 300.0;
        let velocity = options.direction.normalize() * BULLET_SPEED;
        let pos = options.ellipse_pos();
        let size = 5.;
        BulletBundle {
            damage: Damage(options.damage),
            pierce: PierceChance(options.pierce),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: YELLOW.into(),
                    custom_size: Some(Vec2::new(size, size)),
                    ..Default::default()
                },
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            velocity: Velocity::linear(velocity),
            collider: Collider::cuboid(size / 2., size / 2.),
            ..Default::default()
        }
    }
}

pub struct BulletOptions {
    pos: Vec3,
    damage: u16,
    pierce: f32,
    direction: Vect,
    size: Vec2,
}

impl BulletOptions {
    pub fn new(
        player_pos: Vec3,
        damage: u16,
        pierce: f32,
        player_size: Vec2,
        target: Vec3,
    ) -> Self {
        let dir = target - player_pos;
        BulletOptions {
            pos: player_pos,
            damage,
            pierce,
            direction: Vect::new(dir.x, dir.y),
            size: player_size,
        }
    }

    ///  Retrieve the pos of the bullet, according to an Ellipse around the player
    fn ellipse_pos(&self) -> Vec3 {
        let angle = Vec2::X.angle_between(self.direction);
        let x = angle.cos() * SQRT_2 * self.size.x / 2.0 + self.pos.x;
        let y = angle.sin() * SQRT_2 * self.size.y / 2.0 + self.pos.y;
        Vec3::new(x, y, 20.)
    }
}
