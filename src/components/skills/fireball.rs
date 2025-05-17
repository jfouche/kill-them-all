use super::{Skill, SkillBook, SkillOfBook};
use crate::components::{
    damage::BaseHitDamageRange,
    equipment::weapon::BaseAttackSpeed,
    item::{ItemDescriptor, ItemRarity},
};
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(SkillBook, Name::new("FireBallLauncherBook"))]
pub struct FireBallLauncherBook;

impl ItemDescriptor for FireBallLauncherBook {
    fn title(&self) -> String {
        "Fire ball launcher".into()
    }

    fn description(&self) -> String {
        "Launch fire ball".into()
    }

    fn tile_index(&self, _rarity: ItemRarity) -> usize {
        38
    }
}

impl SkillOfBook for FireBallLauncherBook {
    type Skill = FireBallLauncher;
}

#[derive(Component, Default)]
#[require(
    Skill,
    Name::new("FireBallLauncher"),
    BaseHitDamageRange::new(1., 2.),
    BaseAttackSpeed(1.0)
)]
pub struct FireBallLauncher;
