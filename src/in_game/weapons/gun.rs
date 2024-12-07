use crate::{components::*, in_game::GameRunningSet};
use bevy::{color::palettes::css::YELLOW, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Gun;

const BASE_ATTACK_SPEED: f32 = 1.2;

pub fn gun() -> impl Bundle {
    (
        Gun,
        Name::new("Gun"),
        WeaponBundle::new(DamageRange(1. ..=2.), BASE_ATTACK_SPEED),
    )
}

#[derive(Component)]
pub struct Bullet;

#[derive(Bundle)]
struct BulletBundle {
    tag: Bullet,
    name: Name,
    ammo: AmmoBundle,
    sprite: Sprite,
    transform: Transform,
}

impl Default for BulletBundle {
    fn default() -> Self {
        BulletBundle {
            tag: Bullet,
            name: Name::new("Bullet"),
            ammo: AmmoBundle::default(),
            sprite: Sprite::default(),
            transform: Transform::default(),
        }
    }
}

const BULLET_SPEED: f32 = 300.0;

impl BulletBundle {
    pub fn new(options: BulletOptions) -> Self {
        let size = 5.;

        let ammo_config = AmmoConfig {
            damage: options.damage,
            pierce: PierceChance(options.pierce),
            collider: Collider::cuboid(size / 2., size / 2.),
            velocity: Velocity::linear(options.direction.normalize() * BULLET_SPEED),
        };

        BulletBundle {
            ammo: AmmoBundle::new(ammo_config),
            sprite: Sprite {
                color: YELLOW.into(),
                custom_size: Some(Vec2::new(size, size)),
                ..Default::default()
            },
            transform: Transform::from_translation(options.player_pos),
            ..Default::default()
        }
    }
}

struct BulletOptions {
    player_pos: Vec3,
    damage: Damage,
    pierce: f32,
    direction: Vect,
}

impl BulletOptions {
    fn new(player_pos: Vec3, damage: Damage, pierce: f32, target: Vec3) -> Self {
        let dir = target - player_pos;
        BulletOptions {
            player_pos,
            damage,
            pierce,
            direction: Vect::new(dir.x, dir.y),
        }
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
                        damage_range.gen(&mut rng),
                        **pierce,
                        nearest,
                    )));
                }
            }
        }
    }
}
