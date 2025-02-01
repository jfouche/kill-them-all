use super::damage::Damage;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

/// Add life to [crate::components::character::BaseLife]
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

/// Increase [crate::components::character::BaseLife] (after applying [MoreLife])
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct IncreaseMaxLife(pub f32);

impl IncreaseMaxLife {
    pub fn reset(&mut self) {
        self.0 = 0.;
    }

    pub fn add(&mut self, incr: &IncreaseMaxLife) {
        self.0 += incr.0;
    }
}

impl std::fmt::Display for IncreaseMaxLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Increase {:.0}% maximum life", self.0)
    }
}

/// Life regenration per second
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct LifeRegen(pub f32);

impl LifeRegen {
    pub fn reset(&mut self) {
        self.0 = 0.;
    }

    pub fn add(&mut self, regen: &LifeRegen) {
        self.0 += regen.0;
    }
}

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
    pub fn reset(&mut self) {
        self.0 = 0.;
    }

    pub fn init(&mut self, base: &BaseArmour) {
        self.0 = base.0;
    }

    pub fn add(&mut self, armour: &Armour) {
        self.0 += armour.0;
    }

    pub fn more(&mut self, more: &MoreArmour) {
        self.0 += more.0;
    }

    pub fn increase(&mut self, increase: &IncreaseArmour) {
        self.0 *= 1. + increase.0 / 100.;
    }

    pub fn mitigate(&self, damage: Damage) -> Damage {
        let d = (5. * *damage * *damage) / (self.0 + 5. * *damage);
        Damage(d)
    }
}

/// Base equipment [Armour]
#[derive(Component, Default, Deref, Reflect)]
#[require(Armour)]
pub struct BaseArmour(pub f32);

/// Add armour to base [BaseArmour]
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct MoreArmour(pub f32);

/// Increase armour to base [BaseArmour], after applying [MoreArmour]
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct IncreaseArmour(pub f32);

/// Increase [crate::components::character::BaseMovementSpeed]
#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct IncreaseMovementSpeed(pub f32);

impl IncreaseMovementSpeed {
    pub fn reset(&mut self) {
        self.0 = 0.;
    }

    pub fn add(&mut self, increase: &IncreaseMovementSpeed) {
        self.0 += increase.0;
    }
}

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

impl IncreaseAttackSpeed {
    pub fn reset(&mut self) {
        self.0 = 0.;
    }

    pub fn add(&mut self, increase: &IncreaseAttackSpeed) {
        self.0 += increase.0;
    }
}

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
    pub fn reset(&mut self) {
        self.0 = 0.;
    }

    pub fn add(&mut self, pierce: &PierceChance) {
        self.0 += pierce.0;
    }

    pub fn try_pierce(&mut self, rng: &mut ThreadRng) -> bool {
        if rng.random_range(0. ..100.) < **self {
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

/// Add damage to all [crate::components::equipment::weapon::Weapon]s of a
/// [crate::components::character::Character]
#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct MoreDamage(pub f32);

impl MoreDamage {
    pub fn reset(&mut self) {
        self.0 = 0.;
    }

    pub fn add(&mut self, damage: &MoreDamage) {
        self.0 += damage.0;
    }
}

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

/// Increase damage to all [crate::components::equipment::Weapon]s, after applying [MoreDamage]
#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct IncreaseDamage(pub f32);

impl IncreaseDamage {
    pub fn reset(&mut self) {
        self.0 = 0.;
    }

    pub fn add(&mut self, increase: &IncreaseDamage) {
        self.0 += increase.0;
    }
}

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

impl IncreaseAreaOfEffect {
    pub fn reset(&mut self) {
        self.0 = 0.;
    }

    pub fn add(&mut self, increase: &IncreaseAreaOfEffect) {
        self.0 += increase.0;
    }
}

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
