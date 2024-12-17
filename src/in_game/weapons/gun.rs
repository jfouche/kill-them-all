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
    q_player: Query<(&Transform, &PierceChance), With<Player>>,
    weapons: Query<(&AttackTimer, &DamageRange, &Parent), With<Gun>>,
    q_monsters: Query<&Transform, With<Monster>>,
) {
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
                    commands.spawn((
                        Bullet,
                        *damage_range,
                        *pierce,
                        Transform::from_translation(player),
                        Velocity::linear((nearest - player).xy().normalize() * BULLET_SPEED),
                    ));
                }
            }
        }
    }
}
