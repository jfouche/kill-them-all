use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use super::rng_provider::{Generator, RngKindProvider};

pub trait Armor {
    fn armor(&self) -> f32;
}

// ==================================================================
// Helmet

#[derive(Component, Clone, Reflect)]
pub enum Helmet {
    None,
    NormalHelmet(NormalHelmet),
}

#[derive(Copy, Clone, Reflect)]
pub struct NormalHelmet {
    pub armor: f32,
}

impl Armor for Helmet {
    fn armor(&self) -> f32 {
        match self {
            Helmet::None => 0.,
            Helmet::NormalHelmet(helmet) => helmet.armor,
        }
    }
}

// ==================================================================
// BodyArmour

#[derive(Component, Clone, Reflect)]
pub enum BodyArmour {
    None,
    NormalBodyArmour(NormalBodyArmour),
}

#[derive(Copy, Clone, Reflect)]
pub struct NormalBodyArmour {
    pub armor: f32,
}

impl Armor for BodyArmour {
    fn armor(&self) -> f32 {
        match self {
            BodyArmour::None => 0.,
            BodyArmour::NormalBodyArmour(body_armour) => body_armour.armor,
        }
    }
}

// ==================================================================
// EquipmentProvider

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentKind {
    NormalHelmet,
    NormalBodyArmour,
}

impl Generator<Equipment> for EquipmentKind {
    fn generate(&self, rng: &mut ThreadRng) -> Equipment {
        match self {
            EquipmentKind::NormalHelmet => Equipment::Helmet(Helmet::NormalHelmet(NormalHelmet {
                armor: rng.gen_range(1..2) as f32,
            })),
            EquipmentKind::NormalBodyArmour => {
                Equipment::BodyArmour(BodyArmour::NormalBodyArmour(NormalBodyArmour {
                    armor: rng.gen_range(1..2) as f32,
                }))
            }
        }
    }
}

#[derive(Component)]
pub enum Equipment {
    Helmet(Helmet),
    BodyArmour(BodyArmour),
}

#[derive(Deref, DerefMut)]
pub struct EquipmentProvider(RngKindProvider<EquipmentKind, Equipment>);

impl EquipmentProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::<EquipmentKind, Equipment>::default();
        provider.add(EquipmentKind::NormalHelmet, 40);
        provider.add(EquipmentKind::NormalBodyArmour, 40);

        EquipmentProvider(provider)
    }
}
