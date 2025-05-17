use super::{Skill, SkillBook, SkillOfBook};
use crate::components::{
    damage::{BaseHitDamageRange, Projectile},
    equipment::weapon::BaseAttackSpeed,
    item::{ItemDescriptor, ItemRarity},
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
/// [SkillBook] for [ShurikenLauncher]s
///
#[derive(Component, Default)]
#[require(SkillBook, Name::new("ShurikenLauncherBook"))]
pub struct ShurikenLauncherBook;

impl ItemDescriptor for ShurikenLauncherBook {
    fn title(&self) -> String {
        "Shuriken launcher".into()
    }

    fn description(&self) -> String {
        "Launch shurikens".into()
    }

    fn tile_index(&self, _rarity: ItemRarity) -> usize {
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
#[require(Name::new("Shuriken"), Projectile, Sprite, Collider::ball(8.))]
pub struct Shuriken;
