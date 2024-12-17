use super::*;
use bevy::prelude::*;
use rand::Rng;

/// Required components for all characters
#[derive(Component, Default)]
#[require(
    BaseLife,
    BaseMovementSpeed,
    IncreaseAttackSpeed,
    PierceChance,
    Armour,
    MoreDamage,
    IncreaseDamage,
    Transform,
    RigidBody(|| RigidBody::Dynamic),
    Velocity,
    Collider,
    CollisionGroups,
    LockedAxes(|| LockedAxes::ROTATION_LOCKED)
)]
pub struct Character;

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

    pub fn hit(&mut self, damage: Damage) {
        if damage.0 > self.0 {
            self.0 = 0.;
        } else {
            self.0 -= damage.0;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.0 == 0.0
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

#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct LifeRegen(pub f32);

impl std::fmt::Display for LifeRegen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Regenerate {:.0} lifes per sec", self.0)
    }
}

/// Add life to the [BaseLife]
#[derive(Component, Default, Clone, Copy, Deref, Debug, Reflect)]
pub struct MoreLife(pub f32);

impl std::fmt::Display for MoreLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0} to maximum life", self.0)
    }
}

/// Increase [BaseLife] (after applying [MoreLife])
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct IncreaseMaxLife(pub f32);

impl std::fmt::Display for IncreaseMaxLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Increase {:.0}% maximum life", self.0)
    }
}

/// Armour
#[derive(Component, Clone, Copy, Default, Deref, DerefMut, Debug, Reflect)]
pub struct Armour(pub f32);

impl std::fmt::Display for Armour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0} Armour", self.0)
    }
}

impl Armour {
    pub fn mitigate(&self, damage: Damage) -> Damage {
        let d = (5. * *damage) / (self.0 + 5. * *damage);
        Damage(d)
    }
}

/// Add armour to base [Armour]
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct MoreArmour(pub f32);

#[derive(Component, Default, Deref, Reflect)]
#[require(MovementSpeed, IncreaseMovementSpeed)]
pub struct BaseMovementSpeed(pub f32);

/// Caculated movement speed, based on [BaseMovementSpeed] and [IncreaseMovementSpeed]
#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct MovementSpeed(pub f32);

/// Increase [BaseMovementSpeed]
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct IncreaseMovementSpeed(pub f32);

impl std::fmt::Display for IncreaseMovementSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:.0}% movement speed", self.0)
    }
}

#[derive(Component, Default, Clone, Copy, Debug, Deref, DerefMut, Reflect)]
pub struct IncreaseAttackSpeed(pub f32);

impl std::fmt::Display for IncreaseAttackSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Add +{:.0}% attack speed", self.0)
    }
}

/// Pierce chance
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct PierceChance(pub f32);

impl PierceChance {
    pub fn try_pierce(&mut self) -> bool {
        if rand::thread_rng().gen_range(0. ..100.) < **self {
            self.0 -= 100.;
            true
        } else {
            false
        }
    }
}

impl std::fmt::Display for PierceChance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:.0}% pierce chance", **self)
    }
}

/// Add damage to all [Weapon]s of a [Character]
#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct MoreDamage(pub f32);

impl std::fmt::Display for MoreDamage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:.0} more damage", **self)
    }
}

/// Increase damage to all [Weapon]s, after applying [MoreDamage]
#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct IncreaseDamage(pub f32);

impl std::fmt::Display for IncreaseDamage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:.0}% increase damage", **self)
    }
}

/// Event to notify a character was hit
#[derive(Event)]
pub struct HitEvent {
    pub damage: Damage,
}

/// Event to notify a character loose life
#[derive(Event, Deref)]
pub struct LooseLifeEvent(pub Damage);

/// Event to notify a character is dying.
#[derive(Event)]
pub struct CharacterDyingEvent;

/// Event to notify a character has died
///
/// The entity will be despawn when receiving this event
#[derive(Event)]
pub struct CharacterDiedEvent;
