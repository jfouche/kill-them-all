use super::*;
use bevy::{color::palettes::css::YELLOW, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::{f32::consts::SQRT_2, time::Duration};

#[derive(Component)]
pub struct Weapon {
    attack_speed: f32,
    damage_min: u16,
    damage_max: u16,
    timer: Timer,
    ready: bool,
}

impl Weapon {
    pub fn new(attack_per_second: f32, damage_min: u16, damage_max: u16) -> Self {
        Weapon {
            attack_speed: attack_per_second,
            damage_min,
            damage_max,
            timer: Timer::from_seconds(1. / attack_per_second, TimerMode::Repeating),
            ready: false,
        }
    }

    pub fn attack(&mut self) -> u16 {
        self.ready = false;
        rand::thread_rng().gen_range(self.damage_min..=self.damage_max)
    }

    pub fn tick(&mut self, delta: Duration, player_attack_speed_increases: f32) -> &Timer {
        let attack_speed = self.attack_speed * (1. + player_attack_speed_increases / 100.);
        self.timer
            .set_duration(Duration::from_secs_f32(1. / attack_speed));
        if self.timer.tick(delta).just_finished() {
            self.ready = true;
        }
        &self.timer
    }

    pub fn ready(&self) -> bool {
        self.ready
    }
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.damage_min, self.damage_max)
    }
}

#[derive(Component)]
pub struct Bullet;

#[derive(Bundle)]
pub struct BulletBundle {
    tag: Bullet,
    name: Name,
    damage: Damage,
    sprite: SpriteBundle,
    body: RigidBody,
    velocity: Velocity,
    collider: Collider,
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
            sprite: SpriteBundle::default(),
            body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            collider: Collider::default(),
            collision_groups: CollisionGroups::new(GROUP_BULLET, Group::ALL & !GROUP_BONUS),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

impl BulletBundle {
    pub fn new(options: BulletOptions) -> Self {
        const BULLET_SPEED: f32 = 25.0;
        let velocity = options.direction.normalize() * BULLET_SPEED;
        let pos = options.ellipse_pos();
        let size = 0.3;
        BulletBundle {
            damage: Damage(options.damage),
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

    ///  Retrieve the pos of the bullet, according to an Ellipse around the player
    fn ellipse_pos(&self) -> Vec3 {
        let angle = Vec2::X.angle_between(self.direction);
        let x = angle.cos() * SQRT_2 * self.size.x / 2.0 + self.pos.x;
        let y = angle.sin() * SQRT_2 * self.size.y / 2.0 + self.pos.y;
        Vec3::new(x, y, 20.)
    }
}

#[derive(Component, Default, Deref)]
pub struct Damage(pub u16);
