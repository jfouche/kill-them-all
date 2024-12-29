use super::Skill;
use crate::components::*;
use bevy::{color::palettes::css::YELLOW, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
#[require(
    Skill,
    Name(|| Name::new("FireBallLauncher")),
    BaseHitDamageRange(|| BaseHitDamageRange::new(1., 2.)),
    BaseAttackSpeed(|| BaseAttackSpeed(1.2))
)]
pub struct FireBallLauncher;

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

const FIREBALL_SIZE: f32 = 5.0;

impl SkillUI for FireBallLauncher {
    fn title() -> String {
        "Fire ball launcher".into()
    }

    fn label() -> String {
        "Launch fire ball".into()
    }

    fn tile_index() -> usize {
        150
    }
}
