use super::{SkillBook, SkillUI};
use crate::components::{damage::BaseHitDamageRange, equipment::weapon::BaseAttackSpeed};
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(
    SkillBook,
    Name::new("FireBallLauncher"),
    BaseHitDamageRange::new(1., 2.),
    BaseAttackSpeed(1.0)
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
        38
    }
}
