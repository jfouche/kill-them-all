use crate::{
    components::*,
    in_game::{GameRunningSet, GameState},
};
use bevy::{color::palettes::css::YELLOW, prelude::*};
use bevy_rapier2d::prelude::*;

const FIREBALL_SPEED: f32 = 300.0;
const FIREBALL_SIZE: f32 = 5.0;

/// The [FireBallLauncher] projectile
#[derive(Component)]
#[require(
    Name(|| Name::new("FireBall")),
    Projectile,
    Collider(|| Collider::cuboid(FIREBALL_SIZE / 2., FIREBALL_SIZE / 2.)),
    Sprite(|| Sprite {
        color: YELLOW.into(),
        custom_size: Some(Vec2::new(FIREBALL_SIZE, FIREBALL_SIZE)),
        ..Default::default()
    }),
)]
struct FireBall;

pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::InGame), despawn_all::<FireBall>)
            .add_systems(Update, cast_fireball.in_set(GameRunningSet::EntityUpdate));
    }
}

fn cast_fireball(
    mut commands: Commands,
    weapons: Query<(&AttackTimer, &HitDamageRange, &Parent), With<FireBallLauncher>>,
    characters: Query<(&Transform, &PierceChance, &Target), With<Character>>,
) {
    for (timer, hit_damage_range, parent) in &weapons {
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

                const MAX_DISTANCE: f32 = 200.;
                if let Some(target_pos) = nearest_target {
                    if gunner.distance(target_pos) < MAX_DISTANCE {
                        commands.spawn((
                            FireBall,
                            *hit_damage_range,
                            DamagerParams {
                                transform: Transform::from_translation(gunner),
                                collision_groups: Damager::collision_groups(*target),
                            },
                            ProjectileParams {
                                pierce_chance: *pierce,
                                velocity: Velocity::linear(
                                    (target_pos - gunner).xy().normalize() * FIREBALL_SPEED,
                                ),
                            },
                        ));
                    }
                }
            }
        }
    }
}
