use crate::{components::*, in_game::GameRunningSet};
use bevy::{color::palettes::css::YELLOW, prelude::*};
use bevy_rapier2d::prelude::*;
use std::f32::consts::SQRT_2;

#[derive(Component)]
pub struct Gun;

const BASE_ATTACK_SPEED: f32 = 1.2;

pub fn gun() -> impl Bundle {
    (
        Gun,
        Weapon,
        Name::new("Gun"),
        DamageRange(1. ..=2.),
        BaseAttackSpeed(BASE_ATTACK_SPEED),
        AttackTimer(Timer::from_seconds(
            1. / BASE_ATTACK_SPEED,
            TimerMode::Repeating,
        )),
    )
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

const BULLET_SPEED: f32 = 300.0;

impl BulletBundle {
    pub fn new(options: BulletOptions) -> Self {
        let velocity = options.direction.normalize() * BULLET_SPEED;
        let pos = options.ellipse_pos();
        let size = 5.;
        BulletBundle {
            damage: options.damage,
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
    player_pos: Vec3,
    player_size: Vec2,
    damage: Damage,
    pierce: f32,
    direction: Vect,
}

impl BulletOptions {
    pub fn new(
        player_pos: Vec3,
        player_size: Vec2,
        damage: Damage,
        pierce: f32,
        target: Vec3,
    ) -> Self {
        let dir = target - player_pos;
        BulletOptions {
            player_pos,
            player_size,
            damage,
            pierce,
            direction: Vect::new(dir.x, dir.y),
        }
    }

    ///  Retrieve the pos of the bullet, according to an Ellipse around the player
    fn ellipse_pos(&self) -> Vec3 {
        let angle = Vec2::X.angle_between(self.direction);
        let x = angle.cos() * SQRT_2 * self.player_size.x / 2.0 + self.player_pos.x;
        let y = angle.sin() * SQRT_2 * self.player_size.y / 2.0 + self.player_pos.y;
        Vec3::new(x, y, 20.)
    }
}

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gun_fires.in_set(GameRunningSet::EntityUpdate));
    }
}

fn gun_fires(
    mut commands: Commands,
    q_player: Query<(&Transform, &PierceChance), With<Player>>,
    weapons: Query<(&AttackTimer, &DamageRange, &Parent), With<Gun>>,
    q_monsters: Query<&Transform, With<Monster>>,
) {
    let mut rng = rand::thread_rng();
    for (timer, damage_range, parent) in &weapons {
        if timer.just_finished() {
            if let Ok((player, pierce)) = q_player.get(**parent) {
                let player = player.translation;
                // Get the nearest monster
                let nearest_monster = q_monsters
                    .iter()
                    .map(|transform| transform.translation)
                    .reduce(|nearest, other| {
                        if player.distance(other) < player.distance(nearest) {
                            other // new nearest
                        } else {
                            nearest
                        }
                    });
                if let Some(nearest) = nearest_monster {
                    commands.spawn(BulletBundle::new(BulletOptions::new(
                        player,
                        PLAYER_SIZE,
                        damage_range.gen(&mut rng),
                        **pierce,
                        nearest,
                    )));
                }
            }
        }
    }
}
