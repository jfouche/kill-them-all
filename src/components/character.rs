use super::*;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(
    BaseLife,
    BaseMovementSpeed,
    IncreaseAttackSpeed,
    PierceChance,
    Armour,
    Transform,
    RigidBody(|| RigidBody::Dynamic),
    Velocity,
    Collider,
    CollisionGroups,
    LockedAxes(|| LockedAxes::ROTATION_LOCKED)
)]
pub struct Character;
