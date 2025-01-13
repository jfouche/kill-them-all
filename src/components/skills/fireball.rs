use super::Skill;
use crate::components::*;
use bevy::prelude::*;

#[derive(Component)]
#[require(
    Skill,
    Name(|| Name::new("FireBallLauncher")),
    BaseHitDamageRange(|| BaseHitDamageRange::new(1., 2.)),
    BaseAttackSpeed(|| BaseAttackSpeed(1.2))
)]
pub struct FireBallLauncher;

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
