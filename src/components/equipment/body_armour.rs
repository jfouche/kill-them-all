use super::*;
use crate::components::rng_provider::*;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Component, Clone, Reflect)]
pub enum BodyArmour {
    None,
    Normal(NormalBodyArmour),
    Magic(MagicBodyArmour),
}

#[derive(Copy, Clone, Reflect)]
pub struct NormalBodyArmour {
    pub armor: f32,
}

impl NormalBodyArmour {
    pub fn generate(rng: &mut ThreadRng) -> Self {
        NormalBodyArmour {
            armor: rng.gen_range(1..=2) as f32,
        }
    }
}

#[derive(Clone, Reflect)]
pub struct MagicBodyArmour {
    pub base: NormalBodyArmour,
    pub affix: BodyArmourAffix,
}

impl MagicBodyArmour {
    pub fn generate(rng: &mut ThreadRng) -> Self {
        let mut affix_provider = BodyArmourAffixProvider::new();
        MagicBodyArmour {
            base: NormalBodyArmour::generate(rng),
            affix: affix_provider
                .gen()
                .expect("BodyArmourAffixProvider should provide at least 1 affix"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BodyArmourAffixKind {
    AddLife,
    AddArmour,
}

#[derive(Clone, Reflect)]
pub enum BodyArmourAffix {
    AddLife(f32),
    AddArmour(f32),
}

impl std::fmt::Display for BodyArmour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyArmour::None => Ok(()),
            BodyArmour::Normal(body_armour) => {
                write!(f, "Body armour : +{} armour", body_armour.armor as u16)
            }
            BodyArmour::Magic(body_armour) => write!(
                f,
                "Body armour : +{} armour\n{}",
                body_armour.base.armor as u16, body_armour.affix
            ),
        }
    }
}

impl std::fmt::Display for BodyArmourAffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyArmourAffix::AddArmour(val) => write!(f, "Item add +{} armour", *val as u16),
            BodyArmourAffix::AddLife(val) => write!(f, "Item add +{} life", *val as u16),
        }
    }
}

impl Generator<BodyArmourAffix> for BodyArmourAffixKind {
    fn generate(&self, rng: &mut ThreadRng) -> BodyArmourAffix {
        match self {
            BodyArmourAffixKind::AddArmour => {
                BodyArmourAffix::AddArmour(rng.gen_range(2..=5) as f32)
            }
            BodyArmourAffixKind::AddLife => BodyArmourAffix::AddLife(rng.gen_range(5..=20) as f32),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct BodyArmourAffixProvider(RngKindProvider<BodyArmourAffixKind, BodyArmourAffix>);

impl BodyArmourAffixProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(BodyArmourAffixKind::AddArmour, 20);
        provider.add(BodyArmourAffixKind::AddLife, 20);
        BodyArmourAffixProvider(provider)
    }
}

impl Armor for BodyArmour {
    fn armor(&self) -> f32 {
        match self {
            BodyArmour::None => 0.,
            BodyArmour::Normal(body_armour) => body_armour.armor,
            BodyArmour::Magic(body_armour) => body_armour.base.armor,
        }
    }
}

impl MoreLife for BodyArmour {
    fn more_life(&self) -> f32 {
        match self {
            BodyArmour::None => 0.,
            BodyArmour::Normal(_) => 0.,
            BodyArmour::Magic(body_armour) => match body_armour.affix {
                BodyArmourAffix::AddLife(life) => life,
                _ => 0.,
            },
        }
    }
}