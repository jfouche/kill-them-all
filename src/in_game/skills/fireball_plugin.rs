use crate::{
    components::{
        affix::PierceChance,
        character::{Character, Target},
        damage::{Damager, DamagerParams, HitDamageRange, Projectile, ProjectileParams},
        despawn_all,
        skills::{fireball::FireBallLauncher, ActivateSkill},
        world_map::LAYER_DAMAGER,
    },
    schedule::GameState,
};
use avian2d::prelude::*;
use bevy::{color::palettes::css::YELLOW, prelude::*};

const FIREBALL_SPEED: f32 = 300.0;
const FIREBALL_SIZE: f32 = 5.0;

/// The [FireBallLauncher] projectile
#[derive(Component)]
#[require(
    Name::new("FireBall"),
    Projectile,
    Collider::rectangle(FIREBALL_SIZE, FIREBALL_SIZE),
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
                    velocity: LinearVelocity(velocity),
                },
            ));
        }
    }
}
