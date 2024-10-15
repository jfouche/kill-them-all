use super::*;
use crate::components::rng_provider::*;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Default, Clone, Reflect)]
pub enum Helmet {
    #[default]
    None,
    Normal(NormalHelmet),
    Magic(MagicHelmet),
}

#[derive(Copy, Clone, Reflect)]
pub struct NormalHelmet {
    pub armour: f32,
}

impl NormalHelmet {
    pub fn generate(rng: &mut ThreadRng) -> Self {
        NormalHelmet {
            armour: rng.gen_range(1..=2) as f32,
        }
    }
}

#[derive(Clone, Reflect)]
pub struct MagicHelmet {
    pub base: NormalHelmet,
    pub affix: HelmetAffix,
}

impl MagicHelmet {
    pub fn generate(rng: &mut ThreadRng) -> Self {
        let mut affix_provider = HelmetAffixProvider::new();
        MagicHelmet {
            base: NormalHelmet::generate(rng),
            affix: affix_provider
                .gen()
                .expect("HelmetAffixProvider should provide at least 1 affix"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum HelmetAffixKind {
    AddLife,
    AddArmour,
}

#[derive(Clone, Reflect)]
pub enum HelmetAffix {
    AddLife(f32),
    AddArmour(f32),
}

impl std::fmt::Display for Helmet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Helmet::None => Ok(()),
            Helmet::Normal(helmet) => write!(f, "Helmet : +{:.0} armour", helmet.armour),
            Helmet::Magic(helmet) => write!(
                f,
                "Helmet : +{:.0} armour\n{}",
                helmet.base.armour, helmet.affix
            ),
        }
    }
}
impl std::fmt::Display for HelmetAffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HelmetAffix::AddArmour(val) => write!(f, "Item add +{:.0} armour", *val),
            HelmetAffix::AddLife(val) => write!(f, "Item add +{:.0} life", *val),
        }
    }
}

impl Generator<HelmetAffix> for HelmetAffixKind {
    fn generate(&self, rng: &mut ThreadRng) -> HelmetAffix {
        match self {
            HelmetAffixKind::AddArmour => HelmetAffix::AddArmour(rng.gen_range(2..=5) as f32),
            HelmetAffixKind::AddLife => HelmetAffix::AddLife(rng.gen_range(5..=20) as f32),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct HelmetAffixProvider(RngKindProvider<HelmetAffixKind, HelmetAffix>);

impl HelmetAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(HelmetAffixKind::AddArmour, 20);
        provider.add(HelmetAffixKind::AddLife, 20);
        HelmetAffixProvider(provider)
    }
}

impl ProvideArmour for Helmet {
    fn armour(&self) -> f32 {
        match self {
            Helmet::None => 0.,
            Helmet::Normal(helmet) => helmet.armour,
            Helmet::Magic(helmet) => helmet.base.armour,
        }
    }
}

impl ProvideMoreLife for Helmet {
    fn more_life(&self) -> f32 {
        match self {
            Helmet::None => 0.,
            Helmet::Normal(_) => 0.,
            Helmet::Magic(helmet) => match helmet.affix {
                HelmetAffix::AddLife(life) => life,
                _ => 0.,
            },
        }
    }
}

impl ProvideIncreaseMaxLife for Helmet {
    fn increase_max_life(&self) -> f32 {
        0.
    }
}

impl ProvideLifeRegen for Helmet {
    fn life_regen(&self) -> f32 {
        0.
    }
}

impl ProvideIncreaseMovementSpeed for Helmet {
    fn increase_movement_speed(&self) -> f32 {
        0.
    }
}

impl ProvideIncreaseAttackSpeed for Helmet {
    fn increase_attack_speed(&self) -> f32 {
        0.
    }
}

impl ProvidePierceChance for Helmet {
    fn pierce_chance(&self) -> f32 {
        0.
    }
}
