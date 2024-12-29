use super::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Required components for all characters
#[derive(Component, Default)]
#[require(
    Target,
    BaseLife,
    BaseMovementSpeed,
    IncreaseAttackSpeed,
    PierceChance,
    MoreDamage,
    IncreaseDamage,
    IncreaseAreaOfEffect,
    Armour,
    Transform,
    RigidBody(|| RigidBody::Dynamic),
    Velocity,
    Collider,
    CollisionGroups,
    LockedAxes(|| LockedAxes::ROTATION_LOCKED)
)]
pub struct Character;

/// Weapon target
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum Target {
    #[default]
    Monster,
    Player,
}

/// Represent the initial life of a character
#[derive(Component, Default, Deref, Clone, Copy, Reflect)]
#[require(Life, MaxLife, IncreaseMaxLife, LifeRegen)]
pub struct BaseLife(pub f32);

/// Represent current life of a character
#[derive(Component, Default, Deref, DerefMut, Clone, Copy, Debug, Reflect)]
pub struct Life(pub f32);

impl Life {
    pub fn check(&mut self, max: MaxLife) {
        if self.0 > *max {
            self.0 = *max;
        }
    }

    pub fn damage(&mut self, damage: Damage) {
        if damage.0 > self.0 {
            self.0 = 0.;
        } else {
            self.0 -= damage.0;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.0 <= 0.0
    }

    pub fn regenerate(&mut self, life: f32) {
        self.0 += life;
    }
}

/// Represent the max life of a character
///
/// It's calculated with the [BaseLife], [crate::components::MoreLife]s
/// and [crate::components::IncreaseMaxLife]s
#[derive(Component, Default, Deref, DerefMut, Clone, Copy, Reflect)]
pub struct MaxLife(pub f32);

impl std::fmt::Display for MaxLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0} life", self.0)
    }
}

/// Event to notify a character was hit
#[derive(Event)]
pub struct HitEvent {
    pub damage: Damage,
}

/// Base movement speed
#[derive(Component, Default, Deref, Reflect)]
#[require(MovementSpeed, IncreaseMovementSpeed)]
pub struct BaseMovementSpeed(pub f32);

/// Caculated movement speed, based on [BaseMovementSpeed] and [IncreaseMovementSpeed]
#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct MovementSpeed(pub f32);

/// Event to notify a character loose life
#[derive(Event, Deref)]
pub struct LooseLifeEvent(pub Damage);

/// Event to notify a character is dying.
#[derive(Event)]
pub struct CharacterDyingEvent;

/// Event to notify a character has died
///
/// The entity will be despawn when receiving this event
#[derive(Event, Deref)]
pub struct CharacterDiedEvent(pub Entity);
