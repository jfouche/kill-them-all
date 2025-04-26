use super::{SkillGem, SkillUI};
use crate::components::{
    damage::{BaseHitDamageRange, Projectile},
    equipment::weapon::BaseAttackSpeed,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
pub struct ShurikenAssets {
    pub shuriken: Handle<Image>,
}

impl FromWorld for ShurikenAssets {
    fn from_world(world: &mut World) -> Self {
        ShurikenAssets {
            shuriken: world.load_asset("shuriken.png"),
        }
    }
}

///
/// Skill that launch [Shuriken]s
///
#[derive(Component, Default)]
#[require(
    SkillGem,
    Name::new("ShurikenLauncher"),
    BaseHitDamageRange::new(2., 4.),
    BaseAttackSpeed(0.6)
)]
pub struct ShurikenLauncher;

impl SkillUI for ShurikenLauncher {
    fn title() -> String {
        "Shuriken launcher".into()
    }

    fn label() -> String {
        "Launch shurikens".into()
    }

    fn tile_index() -> usize {
        153
    }
}
///
/// A shuriken projectile
///
#[derive(Component)]
#[require(Name::new("Shuriken"), Projectile, Sprite, Collider::ball(8.))]
pub struct Shuriken;
