use super::{Skill, SkillBook, SkillBookUI, SkillOfBook};
use crate::components::{
    damage::{BaseHitDamageRange, Projectile},
    equipment::weapon::BaseAttackSpeed,
};
use avian2d::prelude::*;
use bevy::prelude::*;

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
/// [SkillBook] for [ShurikenLauncher]s
///
#[derive(Component, Default)]
#[require(SkillBook, Name::new("ShurikenLauncherBook"))]
pub struct ShurikenLauncherBook;

impl SkillBookUI for ShurikenLauncherBook {
    fn title() -> String {
        "Shuriken launcher".into()
    }

    fn label() -> String {
        "Launch shurikens".into()
    }

    fn tile_index() -> usize {
        31
    }
}

impl SkillOfBook for ShurikenLauncherBook {
    type Skill = ShurikenLauncher;
}

///
/// [Skill] that launch [Shuriken]s
///
#[derive(Component, Default)]
#[require(
    Skill,
    Name::new("ShurikenLauncher"),
    BaseHitDamageRange::new(2., 4.),
    BaseAttackSpeed(0.6)
)]
pub struct ShurikenLauncher;

///
/// A shuriken projectile
///
#[derive(Component)]
#[require(Name::new("Shuriken"), Projectile, Sprite, Collider::circle(8.))]
pub struct Shuriken;
