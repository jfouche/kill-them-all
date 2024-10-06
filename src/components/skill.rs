use bevy::prelude::*;
use rand::Rng;

#[derive(Bundle, Default)]
pub struct SkillsBundle {
    pub movement_speed: MovementSpeedBundle,
    pub life: LifeBundle,
    pub life_regen: LifeRegen,
    pub attack_speed: IncreaseAttackSpeed,
    pub pierce: PierceChance,
}

// ==================================================================
// MovementSpeed

#[derive(Bundle, Default)]
pub struct MovementSpeedBundle {
    base: BaseMovementSpeed,
    current: MovementSpeed,
}

impl MovementSpeedBundle {
    pub fn new(base: f32) -> Self {
        MovementSpeedBundle {
            base: BaseMovementSpeed(base),
            current: MovementSpeed(base),
        }
    }
}

#[derive(Component, Default, Deref, Reflect)]
pub struct BaseMovementSpeed(f32);

#[derive(Component, Default, Deref, Reflect)]
pub struct MovementSpeed(pub f32);

impl std::fmt::Display for MovementSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ==================================================================
// Life

#[derive(Bundle, Default)]
pub struct LifeBundle {
    base: BaseLife,
    current: Life,
    max: MaxLife,
}

impl LifeBundle {
    pub fn new(life: f32) -> Self {
        LifeBundle {
            base: BaseLife(life),
            current: Life(life),
            max: MaxLife(life),
        }
    }
}

/// Represent the initial life of a character
#[derive(Component, Default, Deref, Clone, Copy, Reflect)]
pub struct BaseLife(f32);

/// Represent current life of a character
#[derive(Component, Default, Deref, Clone, Copy, Reflect)]
pub struct Life(pub f32);

/// Represent the max life of a character
///
/// It's calculated with the [BaseLife] and all [crate::components::Upgrades]
/// and [crate::components::Equipment]s
#[derive(Component, Default, Deref, Clone, Copy, Reflect)]
pub struct MaxLife(pub f32);

impl Life {
    pub fn check(&mut self, max: MaxLife) {
        if self.0 > *max {
            self.0 = *max;
        }
    }

    pub fn hit(&mut self, damage: f32) {
        if damage > self.0 {
            self.0 = 0.;
        } else {
            self.0 -= damage;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.0 == 0.0
    }

    pub fn regenerate(&mut self, life: f32) {
        self.0 += life;
    }
}

// ==================================================================
// LifeRegen

#[derive(Component, Default, Deref, Reflect)]
pub struct LifeRegen(f32);

impl std::fmt::Display for LifeRegen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}% per sec", self.0)
    }
}

// ==================================================================
// AttackSpeed

#[derive(Component, Default, Clone, Copy, Reflect, Deref)]
pub struct IncreaseAttackSpeed(pub f32);

impl std::fmt::Display for IncreaseAttackSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:.0}%", self.0)
    }
}

// ==================================================================
// Pierce

#[derive(Component, Default, Clone, Copy, Deref, Reflect)]
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
        write!(f, "+{:.0}%", **self)
    }
}

// ==================================================================
// HitEvent

// /// Event to notify a monster was hit
// #[derive(Event)]
// pub struct HitEvent {
//     pub entity: Entity,
//     pub damage: Damage,
// }
// }
