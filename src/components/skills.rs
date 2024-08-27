use bevy::prelude::*;
use rand::Rng;

#[derive(Bundle, Default)]

pub struct SkillsBundle {
    pub movement_speed: MovementSpeed,
    pub life: Life,
    pub attack_speed: AttackSpeed,
    pub pierce: PierceChance,
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

    pub fn increases(&mut self, percent: f32) {
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
    life: u16,
    max_life: u16,
    increases: f32,
}

impl Life {
    pub fn new(life: u16) -> Self {
        Life {
            life,
            max_life: life,
            increases: 0.,
        }
    }

    pub fn hit(&mut self, damage: u16) {
        if damage > self.life {
            self.life = 0;
        } else {
            self.life -= damage;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.life == 0
    }

    pub fn life(&self) -> u16 {
        self.life
    }

    pub fn max_life(&self) -> u16 {
        (self.max_life as f32 * (100.0 + self.increases) / 100.0) as u16
    }

    pub fn increases(&mut self, percent: f32) {
        self.increases += percent;
    }

    pub fn regenerate(&mut self, life: u16) {
        self.life = std::cmp::min(self.max_life(), self.life + life);
    }
}

impl std::fmt::Display for Life {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}  (+{}%)",
            self.life(),
            self.max_life(),
            self.increases
        )
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

    pub fn increases(&mut self, percent: f32) {
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
    pub fn increases(&mut self, percent: f32) {
        **self += percent;
    }

    pub fn try_pierce(&mut self) -> bool {
        if rand::thread_rng().gen_range(0. ..100.) < **self {
            **self -= 100.;
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
