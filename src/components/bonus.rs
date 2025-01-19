use super::*;
use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Component, Copy, Clone, Deref, Reflect)]
#[require(
    Name(|| Name::new("Bonus")),
    Sprite,
)]
pub struct Bonus(pub Entity);

/// Provide a random bonus
pub struct BonusProvider;

impl BonusProvider {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> Option<EquipmentEntityInfo> {
        if rng.gen_range(0..100) < 120 {
            EquipmentProvider::new().spawn(commands, rng)
        } else {
            None
        }
    }
}
