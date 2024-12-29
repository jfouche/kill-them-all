use super::Damage;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

/// Add life to the [crate::components::BaseLife]
#[derive(Component, Default, Clone, Copy, Deref, Debug, Reflect)]
pub struct MoreLife(pub f32);

impl std::fmt::Display for MoreLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0} to maximum life", self.0)
    }
}

impl From<u16> for MoreLife {
    fn from(value: u16) -> Self {
        MoreLife(value as f32)
    }
}

/// Increase [crate::components::BaseLife] (after applying [MoreLife])
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct IncreaseMaxLife(pub f32);

impl std::fmt::Display for IncreaseMaxLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Increase {:.0}% maximum life", self.0)
    }
}

/// Life regenration per second
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct LifeRegen(pub f32);

impl std::fmt::Display for LifeRegen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Regenerate {:.0} lifes per sec", self.0)
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

impl From<u16> for Armour {
    fn from(value: u16) -> Self {
        Armour(value as f32)
    }
}

impl Armour {
    pub fn mitigate(&self, damage: Damage) -> Damage {
        let d = (5. * *damage * *damage) / (self.0 + 5. * *damage);
        Damage(d)
    }
}

/// Add armour to base [Armour]
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct MoreArmour(pub f32);

/// Increase [crate::components::BaseMovementSpeed]
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct IncreaseMovementSpeed(pub f32);

impl std::fmt::Display for IncreaseMovementSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:.0}% movement speed", self.0)
    }
}

impl From<u16> for IncreaseMovementSpeed {
    fn from(value: u16) -> Self {
        IncreaseMovementSpeed(value as f32)
    }
}

#[derive(Component, Default, Clone, Copy, Debug, Deref, DerefMut, Reflect)]
pub struct IncreaseAttackSpeed(pub f32);

impl std::fmt::Display for IncreaseAttackSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Add +{:.0}% attack speed", self.0)
    }
}

impl From<u16> for IncreaseAttackSpeed {
    fn from(value: u16) -> Self {
        IncreaseAttackSpeed(value as f32)
    }
}

/// Pierce chance
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct PierceChance(pub f32);

impl From<u16> for PierceChance {
    fn from(value: u16) -> Self {
        PierceChance(value as f32)
    }
}

impl PierceChance {
    pub fn try_pierce(&mut self, rng: &mut ThreadRng) -> bool {
        if rng.gen_range(0. ..100.) < **self {
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

/// Add damage to all [crate::components::Weapon]s of a [crate::components::Character]
#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct MoreDamage(pub f32);

impl std::fmt::Display for MoreDamage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:.0} more damage", **self)
    }
}

impl From<u16> for MoreDamage {
    fn from(value: u16) -> Self {
        MoreDamage(value as f32)
    }
}

/// Increase damage to all [crate::components::Weapon]s, after applying [MoreDamage]
#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct IncreaseDamage(pub f32);

impl std::fmt::Display for IncreaseDamage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:.0}% increase damage", **self)
    }
}

impl From<u16> for IncreaseDamage {
    fn from(value: u16) -> Self {
        IncreaseDamage(value as f32)
    }
}

/// Increase area of effect
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Reflect)]
pub struct IncreaseAreaOfEffect(pub f32);

impl std::fmt::Display for IncreaseAreaOfEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:.0}% increase area of effect", **self)
    }
}

impl From<u16> for IncreaseAreaOfEffect {
    fn from(value: u16) -> Self {
        IncreaseAreaOfEffect(value as f32)
    }
}
