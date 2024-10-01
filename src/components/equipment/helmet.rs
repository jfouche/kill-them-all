use super::*;
use crate::components::rng_provider::*;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Component, Clone, Reflect)]
pub enum Helmet {
    None,
    Normal(NormalHelmet),
    Magic(MagicHelmet),
}

#[derive(Copy, Clone, Reflect)]
pub struct NormalHelmet {
    pub armor: f32,
}

impl NormalHelmet {
    pub fn generate(rng: &mut ThreadRng) -> Self {
        NormalHelmet {
            armor: rng.gen_range(1..=2) as f32,
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
            Helmet::Normal(helmet) => write!(f, "Helmet : +{} armour", helmet.armor as u16),
            Helmet::Magic(helmet) => write!(
                f,
                "Helmet : +{} armour\n{}",
                helmet.base.armor as u16, helmet.affix
            ),
        }
    }
}
impl std::fmt::Display for HelmetAffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HelmetAffix::AddArmour(val) => write!(f, "Item add +{} armour", *val as u16),
            HelmetAffix::AddLife(val) => write!(f, "Item add +{} life", *val as u16),
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

impl Armor for Helmet {
    fn armor(&self) -> f32 {
        match self {
            Helmet::None => 0.,
            Helmet::Normal(helmet) => helmet.armor,
            Helmet::Magic(helmet) => helmet.base.armor,
        }
    }
}

impl MoreLife for Helmet {
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
