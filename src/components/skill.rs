use super::*;
use bevy::prelude::*;
use rand::Rng;

// TODO: move to character module ?
#[derive(Component, Default)]
pub struct Character;

#[derive(Bundle, Default)]
pub struct SkillsBundle {
    pub character: Character,
    pub armour: Armour,
    pub movement_speed: MovementSpeedBundle,
    pub life: LifeBundle,
    pub more_life: MoreLife,
    pub incr_life: IncreaseMaxLife,
    pub life_regen: LifeRegen,
    pub attack_speed: IncreaseAttackSpeed,
    pub pierce: PierceChance,
}

#[derive(Component, Clone, Deref, Reflect)]
pub struct AffixesLabels(pub String);

// ==================================================================
// Armour

#[derive(Component, Clone, Copy, Default, Deref, DerefMut, Debug, Reflect)]
pub struct Armour(pub f32);

impl super::Label for Armour {
    fn label(&self) -> String {
        format!("{:.0} Armour", self.0)
    }
}

impl Armour {
    pub fn mitigate(&self, damage: Damage) -> Damage {
        let d = (5. * *damage) / (self.0 + 5. * *damage);
        Damage(d)
    }
}

// ==================================================================
// MovementSpeed

#[derive(Bundle, Default)]
pub struct MovementSpeedBundle {
    base: BaseMovementSpeed,
    current: MovementSpeed,
    incr: IncreaseMovementSpeed,
}

impl MovementSpeedBundle {
    pub fn new(base: f32) -> Self {
        MovementSpeedBundle {
            base: BaseMovementSpeed(base),
            current: MovementSpeed(base),
            incr: IncreaseMovementSpeed(0.),
        }
    }
}

#[derive(Component, Default, Deref, Reflect)]
pub struct BaseMovementSpeed(f32);

#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct MovementSpeed(pub f32);

impl super::Label for MovementSpeed {
    fn label(&self) -> String {
        format!("Add {:.0}% movement speed", self.0)
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
pub struct BaseLife(pub f32);

/// Represent current life of a character
#[derive(Component, Default, Deref, DerefMut, Clone, Copy, Debug, Reflect)]
pub struct Life(pub f32);

/// Represent the max life of a character
///
/// It's calculated with the [BaseLife], [crate::components::MoreLife]s
/// and [crate::components::IncreaseMaxLife]s
#[derive(Component, Default, Deref, DerefMut, Clone, Copy, Reflect)]
pub struct MaxLife(pub f32);

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

impl super::Label for Life {
    fn label(&self) -> String {
        format!("{:.0} life", self.0)
    }
}

// ==================================================================
// LifeRegen

#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct LifeRegen(pub f32);

impl super::Label for LifeRegen {
    fn label(&self) -> String {
        format!("Regenerate {:.0} lifes per sec", self.0)
    }
}

// ==================================================================
// AttackSpeed

#[derive(Component, Default, Clone, Copy, Debug, Deref, DerefMut, Reflect)]
pub struct IncreaseAttackSpeed(pub f32);

impl super::Label for IncreaseAttackSpeed {
    fn label(&self) -> String {
        format!("Add +{:.0}% attack speed", self.0)
    }
}

// ==================================================================
// Pierce

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

impl super::Label for PierceChance {
    fn label(&self) -> String {
        format!("+{:.0}% pierce chance", **self)
    }
}

// ==================================================================
// HitEvent

/// Event to notify a character was hit
#[derive(Event)]
pub struct HitEvent {
    pub damage: Damage,
}

/// Event to notify a character loose life
#[derive(Event, Deref)]
pub struct LooseLifeEvent(pub Damage);

/// Event to notify a character is dying
#[derive(Event)]
pub struct CharacterDyingEvent;

/// Event to notify a character has died
#[derive(Event)]
pub struct CharacterDiedEvent;
