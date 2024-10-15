use super::*;
use crate::components::rng_provider::*;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Component, Default, Clone, Reflect)]
pub enum Boots {
    #[default]
    None,
    Normal(NormalBoots),
    Magic(MagicBoots),
}

#[derive(Copy, Clone, Reflect)]
pub struct NormalBoots {
    pub armour: f32,
}

impl NormalBoots {
    pub fn generate(rng: &mut ThreadRng) -> Self {
        NormalBoots {
            armour: rng.gen_range(1..=2) as f32,
        }
    }
}

#[derive(Clone, Reflect)]
pub struct MagicBoots {
    pub base: NormalBoots,
    pub affix: BootsAffix,
}

impl MagicBoots {
    pub fn generate(rng: &mut ThreadRng) -> Self {
        let mut affix_provider = BootsAffixProvider::new();
        MagicBoots {
            base: NormalBoots::generate(rng),
            affix: affix_provider
                .gen()
                .expect("BootsAffixProvider should provide at least 1 affix"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BootsAffixKind {
    AddLife,
    AddArmour,
    IncreaseMovementSpeed,
}

#[derive(Clone, Reflect)]
pub enum BootsAffix {
    AddLife(f32),
    AddArmour(f32),
    IncreaseMovementSpeed(f32),
}

impl std::fmt::Display for Boots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Boots::None => Ok(()),
            Boots::Normal(boots) => {
                write!(f, "Boots : +{:.0} armour", boots.armour)
            }
            Boots::Magic(boots) => write!(
                f,
                "Boots : +{:.0} armour\n{}",
                boots.base.armour, boots.affix
            ),
        }
    }
}

impl std::fmt::Display for BootsAffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootsAffix::AddArmour(val) => write!(f, "Item add +{:.0} armour", *val),
            BootsAffix::AddLife(val) => write!(f, "Item add +{:.0} life", *val),
            BootsAffix::IncreaseMovementSpeed(val) => {
                write!(f, "Item increase +{:.0}% movement speed", *val)
            }
        }
    }
}

impl Generator<BootsAffix> for BootsAffixKind {
    fn generate(&self, rng: &mut ThreadRng) -> BootsAffix {
        match self {
            BootsAffixKind::AddArmour => BootsAffix::AddArmour(rng.gen_range(2..=5) as f32),
            BootsAffixKind::AddLife => BootsAffix::AddLife(rng.gen_range(5..=20) as f32),
            BootsAffixKind::IncreaseMovementSpeed => {
                BootsAffix::IncreaseMovementSpeed(rng.gen_range(5..=30) as f32)
            }
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct BootsAffixProvider(RngKindProvider<BootsAffixKind, BootsAffix>);

impl BootsAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(BootsAffixKind::AddArmour, 20);
        provider.add(BootsAffixKind::AddLife, 20);
        provider.add(BootsAffixKind::IncreaseMovementSpeed, 20);
        BootsAffixProvider(provider)
    }
}

impl ProvideArmour for Boots {
    fn armour(&self) -> f32 {
        match self {
            Boots::None => 0.,
            Boots::Normal(boot) => boot.armour,
            Boots::Magic(boot) => boot.base.armour,
        }
    }
}

impl ProvideMoreLife for Boots {
    fn more_life(&self) -> f32 {
        match self {
            Boots::None => 0.,
            Boots::Normal(_) => 0.,
            Boots::Magic(boots) => match boots.affix {
                BootsAffix::AddLife(life) => life,
                _ => 0.,
            },
        }
    }
}

impl ProvideIncreaseMaxLife for Boots {
    fn increase_max_life(&self) -> f32 {
        0.
    }
}
impl ProvideLifeRegen for Boots {
    fn life_regen(&self) -> f32 {
        0.
    }
}

impl ProvideIncreaseMovementSpeed for Boots {
    fn increase_movement_speed(&self) -> f32 {
        match self {
            Boots::None => 0.,
            Boots::Normal(_) => 0.,
            Boots::Magic(boots) => match boots.affix {
                BootsAffix::IncreaseMovementSpeed(life) => life,
                _ => 0.,
            },
        }
    }
}

impl ProvideIncreaseAttackSpeed for Boots {
    fn increase_attack_speed(&self) -> f32 {
        0.
    }
}
impl ProvidePierceChance for Boots {
    fn pierce_chance(&self) -> f32 {
        0.
    }
}
