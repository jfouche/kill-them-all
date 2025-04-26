use super::{
    affix::{
        Armour, IncreaseAreaOfEffect, IncreaseAttackSpeed, IncreaseDamage, IncreaseMaxLife,
        IncreaseMovementSpeed, LifeRegen, MoreDamage, MoreLife, PierceChance,
    },
    damage::Damage,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Required components for all characters
#[derive(Component, Default)]
#[require(
    Target,
    CharacterAction,
    CharacterLevel,
    BaseLife,
    BaseMovementSpeed,
    IncreaseAttackSpeed,
    PierceChance,
    MoreDamage,
    IncreaseDamage,
    IncreaseAreaOfEffect,
    Armour,
    Transform,
    RigidBody::Dynamic,
    Velocity,
    Collider,
    CollisionGroups,
    LockedAxes::ROTATION_LOCKED
)]
pub struct Character;

/// Weapon target
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum Target {
    #[default]
    Monster,
    Player,
}

/// Action the character should do
#[derive(Component, Default, Clone, Copy, Reflect)]
pub enum CharacterAction {
    #[default]
    Stop,
    GoTo(Vec2),
    TakeItem(Entity),
}

impl CharacterAction {
    pub fn stop(&mut self) {
        *self = CharacterAction::Stop;
    }

    pub fn goto(&mut self, pos: Vec2) {
        *self = CharacterAction::GoTo(pos);
    }

    pub fn take_item(&mut self, entity: Entity) {
        *self = CharacterAction::TakeItem(entity)
    }
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
/// It's calculated with the [BaseLife], [crate::components::affix::MoreLife]s
/// and [crate::components::affix::IncreaseMaxLife]s
#[derive(Component, Default, Deref, DerefMut, Clone, Copy, Reflect)]
pub struct MaxLife(pub f32);

impl MaxLife {
    pub fn init(&mut self, base: &BaseLife) {
        self.0 = base.0;
    }

    pub fn more(&mut self, more: &MoreLife) {
        self.0 += more.0;
    }

    pub fn increase(&mut self, increase: &IncreaseMaxLife) {
        self.0 *= 1. + increase.0 / 100.;
    }
}

impl std::fmt::Display for MaxLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0} life", self.0)
    }
}

/// Event to notify a character was hit
#[derive(Event)]
pub struct HitEvent {
    pub damager: Entity,
    pub damage: Damage,
}

/// Base movement speed
#[derive(Component, Default, Deref, Reflect)]
#[require(MovementSpeed, IncreaseMovementSpeed)]
pub struct BaseMovementSpeed(pub f32);

/// Caculated movement speed, based on [BaseMovementSpeed] and [IncreaseMovementSpeed]
#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct MovementSpeed(pub f32);

impl MovementSpeed {
    pub fn init(&mut self, base: &BaseMovementSpeed) {
        self.0 = base.0;
    }

    pub fn increase(&mut self, increase: &IncreaseMovementSpeed) {
        self.0 *= 1. + increase.0 / 100.;
    }
}

/// Character level
#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct CharacterLevel(pub u16);

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
