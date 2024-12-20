use crate::{components::*, in_game::GameRunningSet};
use bevy::{color::palettes::css::YELLOW, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
#[require(
    Weapon,
    Name(|| Name::new("Gun")),
    BaseDamageRange(|| BaseDamageRange::new(1., 2.)),
    BaseAttackSpeed(|| BaseAttackSpeed(1.2))
)]
pub struct Gun;

const BULLET_SPEED: f32 = 300.0;
const BULLET_SIZE: f32 = 5.0;

#[derive(Component)]
#[require(
    Name(|| Name::new("Bullet")),
    Projectile,
    Collider(|| Collider::cuboid(BULLET_SIZE / 2., BULLET_SIZE / 2.)),
    Sprite(|| Sprite {
        color: YELLOW.into(),
        custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
        ..Default::default()
    }),
)]
struct Bullet;

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gun_fires.in_set(GameRunningSet::EntityUpdate));
    }
}

fn gun_fires(
    mut commands: Commands,
    weapons: Query<(&AttackTimer, &DamageRange, &Parent), With<Gun>>,
    characters: Query<(&Transform, &PierceChance, &Target), With<Character>>,
) {
    for (timer, damage_range, parent) in &weapons {
        if timer.just_finished() {
            if let Ok((gunner_pos, pierce, target)) = characters.get(**parent) {
                let gunner = gunner_pos.translation;
                // Get the nearest target
                let nearest_target = characters
                    .iter()
                    .filter(|(_t, _pc, other_target)| *other_target != target)
                    .map(|(transform, _pc, _t)| transform.translation)
                    .reduce(|nearest, other| {
                        if gunner.distance(other) < gunner.distance(nearest) {
                            other // new nearest
                        } else {
                            nearest
                        }
                    });
                if let Some(target_pos) = nearest_target {
                    commands.spawn((
                        Bullet,
                        AmmoParams {
                            damage_range: *damage_range,
                            transform: Transform::from_translation(gunner),
                            collision_groups: Ammo::collision_groups(*target)
                        },
                        ProjectileParams {
                            pierce_chance: *pierce,
                            velocity: Velocity::linear((target_pos - gunner).xy().normalize() * BULLET_SPEED)
                        }
                    ));
                }
            }
        }
    }
}
