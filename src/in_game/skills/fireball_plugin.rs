use crate::{
    components::{
        affix::PierceChance,
        character::{Character, Target},
        damage::{Damager, DamagerParams, HitDamageRange, Projectile, ProjectileParams},
        despawn_all,
        item::update_item_info,
        skills::{
            fireball::{FireBallLauncher, FireBallLauncherBook},
            ActivateSkill,
        },
        world_map::LAYER_DAMAGER,
    },
    schedule::GameState,
};
use bevy::{color::palettes::css::YELLOW, prelude::*};
use bevy_rapier2d::prelude::*;

const FIREBALL_SPEED: f32 = 300.0;
const FIREBALL_SIZE: f32 = 5.0;

/// The [FireBallLauncher] projectile
#[derive(Component)]
#[require(
    Name::new("FireBall"),
    Projectile,
    Collider::cuboid(FIREBALL_SIZE / 2., FIREBALL_SIZE / 2.),
    Sprite {
        color: YELLOW.into(),
        custom_size: Some(Vec2::new(FIREBALL_SIZE, FIREBALL_SIZE)),
        ..Default::default()
    },
)]
struct FireBall;

pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::InGame), despawn_all::<FireBall>)
            .add_observer(update_item_info::<FireBallLauncherBook>())
            .add_observer(cast_fireball);
    }
}

fn cast_fireball(
    trigger: Trigger<ActivateSkill>,
    mut commands: Commands,
    skills: Query<(&HitDamageRange, &ChildOf), With<FireBallLauncher>>,
    characters: Query<(&Transform, &PierceChance, &Target), With<Character>>,
) {
    let (skill_entity, target_pos) = (trigger.0, trigger.1);
    if let Ok((damage_range, child_of)) = skills.get(skill_entity) {
        if let Ok((origin, pierce, target)) = characters.get(child_of.parent()) {
            let origin = origin.translation.xy();
            let velocity = (target_pos - origin).normalize() * FIREBALL_SPEED;
            commands.spawn((
                FireBall,
                *damage_range,
                DamagerParams {
                    transform: Transform::from_translation(origin.extend(LAYER_DAMAGER)),
                    collision_groups: Damager::collision_groups(*target),
                },
                ProjectileParams {
                    pierce_chance: *pierce,
                    velocity: Velocity::linear(velocity),
                },
            ));
        }
    }
}
