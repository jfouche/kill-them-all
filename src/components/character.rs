use bevy::prelude::*;
use rand::Rng;

#[derive(Bundle, Default)]

pub struct SkillsBundle {
    pub movement_speed: MovementSpeed,
    pub life: Life,
    pub life_regen: LifeRegen,
    pub attack_speed: AttackSpeed,
    pub pierce: PierceChance,
}

pub trait Increase {
    /// Increase Self, by `percent` (1.0 is 1%)
    fn increase(&mut self, percent: f32);
}

// ==================================================================
// MovementSpeed

#[derive(Component, Default, Reflect)]
pub struct MovementSpeed {
    speed: f32,
    increases: f32,
}

impl MovementSpeed {
    pub fn new(speed: f32) -> Self {
        MovementSpeed {
            speed,
            increases: 0.0,
        }
    }
    pub fn value(&self) -> f32 {
        self.speed * (100.0 + self.increases) / 100.0
    }
}

impl Increase for MovementSpeed {
    fn increase(&mut self, percent: f32) {
        self.increases += percent;
    }
}

impl std::fmt::Display for MovementSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}  (+{:.0}%)", self.value(), self.increases)
    }
}

// ==================================================================
// Life

#[derive(Component, Default, Reflect)]
pub struct Life {
    life: f32,
    max_life: f32,
    increases: f32,
}

impl Life {
    pub fn new(life: f32) -> Self {
        Life {
            life,
            max_life: life,
            increases: 0.,
        }
    }

    pub fn hit(&mut self, damage: f32) {
        if damage > self.life {
            self.life = 0.;
        } else {
            self.life -= damage;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.life == 0.0
    }

    pub fn life(&self) -> f32 {
        self.life
    }

    pub fn max_life(&self) -> f32 {
        self.max_life * (100.0 + self.increases) / 100.0
    }

    pub fn regenerate(&mut self, life: f32) {
        self.life = self.max_life().min(self.life + life);
    }
}

impl Increase for Life {
    fn increase(&mut self, percent: f32) {
        self.increases += percent;
    }
}

impl std::fmt::Display for Life {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}  (+{}%)",
            self.life().round(),
            self.max_life().round(),
            self.increases
        )
    }
}

// ==================================================================
// LifeRegen

#[derive(Component, Default, Reflect)]
pub struct LifeRegen {
    pub increases: f32,
}

impl Increase for LifeRegen {
    fn increase(&mut self, percent: f32) {
        self.increases += percent;
    }
}

impl std::fmt::Display for LifeRegen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}% per sec", self.increases)
    }
}

// ==================================================================
// AttackSpeed

#[derive(Component, Reflect)]
pub struct AttackSpeed {
    increases: f32,
}

impl Default for AttackSpeed {
    fn default() -> Self {
        AttackSpeed { increases: 0.0 }
    }
}

impl AttackSpeed {
    pub fn value(&self) -> f32 {
        self.increases
    }
}

impl Increase for AttackSpeed {
    fn increase(&mut self, percent: f32) {
        self.increases += percent;
    }
}

impl std::fmt::Display for AttackSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:.0}%", self.increases)
    }
}

// ==================================================================
// Pierce

#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct PierceChance(pub f32);

impl PierceChance {
    pub fn try_pierce(&mut self) -> bool {
        if rand::thread_rng().gen_range(0. ..100.) < **self {
            **self -= 100.;
            true
        } else {
            false
        }
    }
}

impl Increase for PierceChance {
    fn increase(&mut self, percent: f32) {
        **self += percent;
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
