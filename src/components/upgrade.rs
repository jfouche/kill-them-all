use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum UpgradeType {
    IncreaseMaxLife,
    IncreaseLifeRegen,
    IncreaseAttackSpeed,
    IncreasemovementSpeed,
    Pierce,
}

impl UpgradeType {
    fn gen(&self, rng: &mut ThreadRng) -> Upgrade {
        match self {
            UpgradeType::IncreaseMaxLife => Upgrade::IncreaseMaxLife(rng.gen_range(2..10) as f32),
            UpgradeType::IncreaseLifeRegen => {
                Upgrade::IncreaseLifeRegen(rng.gen_range(2..10) as f32)
            }
            UpgradeType::IncreaseAttackSpeed => {
                Upgrade::IncreaseAttackSpeed(rng.gen_range(2..20) as f32)
            }
            UpgradeType::IncreasemovementSpeed => {
                Upgrade::IncreasemovementSpeed(rng.gen_range(2..20) as f32)
            }
            UpgradeType::Pierce => Upgrade::Pierce(rng.gen_range(2..20) as f32),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Upgrade {
    /// Increase max life percentage, 1.0 is 100%
    IncreaseMaxLife(f32),
    /// Increase life regen percentage, 1.0 is 100%
    IncreaseLifeRegen(f32),
    /// Increase attack speed percentage, 1.0 is 100%
    IncreaseAttackSpeed(f32),
    /// Increase movement speed percentage, 1.0 is 100%
    IncreasemovementSpeed(f32),
    // Pierce allow to not despawn when hitting
    Pierce(f32),
}

pub struct UpgradeProvider {
    upgrades: HashMap<UpgradeType, Vec<UpgradeType>>,
    filters: Vec<UpgradeType>,
}

impl UpgradeProvider {
    pub fn new() -> Self {
        let mut provider = UpgradeProvider {
            upgrades: HashMap::new(),
            filters: vec![],
        };
        provider.add(UpgradeType::IncreaseMaxLife, 40);
        provider.add(UpgradeType::IncreaseLifeRegen, 40);
        provider.add(UpgradeType::IncreaseAttackSpeed, 20);
        provider.add(UpgradeType::IncreasemovementSpeed, 40);
        provider.add(UpgradeType::Pierce, 20);

        provider
    }

    fn add(&mut self, upgrade: UpgradeType, weight: usize) -> &mut Self {
        self.upgrades.insert(upgrade, vec![upgrade; weight]);
        self
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
