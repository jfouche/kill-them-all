use super::{Skill, SkillBook, SkillBookUI, SkillOfBook};
use crate::components::{damage::BaseHitDamageRange, equipment::weapon::BaseAttackSpeed};
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(SkillBook, Name::new("FireBallLauncherBook"))]
pub struct FireBallLauncherBook;

impl SkillBookUI for FireBallLauncherBook {
    fn title() -> String {
        "Fire ball launcher".into()
    }

    fn label() -> String {
        "Launch fire ball".into()
    }

    fn tile_index() -> usize {
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
