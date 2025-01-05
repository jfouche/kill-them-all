use super::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Component, Copy, Clone, Deref, Reflect)]
#[require(
    Name(|| Name::new("Bonus")),
    Sprite,
    RigidBody(|| RigidBody::Fixed),
    Collider(bonus_collider),
    CollisionGroups(|| CollisionGroups::new(GROUP_BONUS, GROUP_ALL)),
)]
pub struct Bonus(pub Entity);

fn bonus_collider() -> Collider {
    let half_x = (EQUIPMENT_SIZE.x / 2) as f32;
    let half_y = (EQUIPMENT_SIZE.y / 2) as f32;
    Collider::cuboid(half_x, half_y)
}

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
