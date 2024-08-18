use bevy::prelude::*;
use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};
use std::collections::HashMap;

#[derive(Component)]
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

#[derive(Component)]
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

#[derive(Component)]
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
// Upgrades

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum UpgradeType {
    /// Increase max life percentage, 1.0 is 100%
    IncreaseMaxLife,
    /// Increase attack speed percentage, 1.0 is 100%
    IncreaseAttackSpeed,
    /// Increase movement speed percentage, 1.0 is 100%
    IncreasemovementSpeed,
}

impl UpgradeType {
    fn gen(&self, rng: &mut ThreadRng) -> Upgrade {
        match self {
            UpgradeType::IncreaseMaxLife => Upgrade::IncreaseMaxLife(rng.gen_range(2..10) as f32),
            UpgradeType::IncreaseAttackSpeed => {
                Upgrade::IncreaseAttackSpeed(rng.gen_range(2..20) as f32)
            }
            UpgradeType::IncreasemovementSpeed => {
                Upgrade::IncreasemovementSpeed(rng.gen_range(2..20) as f32)
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Upgrade {
    /// Increase max life percentage, 1.0 is 100%
    IncreaseMaxLife(f32),
    /// Increase attack speed percentage, 1.0 is 100%
    IncreaseAttackSpeed(f32),
    /// Increase movement speed percentage, 1.0 is 100%
    IncreasemovementSpeed(f32),
}

pub struct UpgradeProvider {
    upgrades: HashMap<UpgradeType, Vec<UpgradeType>>,
    filters: Vec<UpgradeType>,
}

impl UpgradeProvider {
    pub fn new() -> Self {
        let mut upgrades = HashMap::new();
        upgrades.insert(
            UpgradeType::IncreaseMaxLife,
            vec![UpgradeType::IncreaseMaxLife; 40],
        );
        upgrades.insert(
            UpgradeType::IncreaseAttackSpeed,
            vec![UpgradeType::IncreaseAttackSpeed; 20],
        );
        upgrades.insert(
            UpgradeType::IncreasemovementSpeed,
            vec![UpgradeType::IncreasemovementSpeed; 40],
        );

        UpgradeProvider {
            upgrades,
            filters: vec![],
        }
    }

    /// generate a rand upgrade, removing the option the select it next time
    pub fn gen(&mut self) -> Option<Upgrade> {
        let mut rng = rand::thread_rng();

        let upgrade_type = self
            .upgrades
            .iter()
            .filter(|(upgrade_type, _v)| !self.filters.contains(upgrade_type))
            .flat_map(|(_upgrade_type, v)| v)
            .choose(&mut rng)?;

        self.filters.push(*upgrade_type);
        let upgrade = upgrade_type.gen(&mut rng);
        Some(upgrade)
    }
}
