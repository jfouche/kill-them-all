use super::*;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(
    Life,
    Armour,
    BaseMovementSpeed,
    IncreaseAttackSpeed,
    PierceChance,
    RigidBody(|| RigidBody::Dynamic),
    Velocity,
    Collider,
    CollisionGroups,
    LockedAxes(|| LockedAxes::ROTATION_LOCKED)
)]
pub struct Character;
