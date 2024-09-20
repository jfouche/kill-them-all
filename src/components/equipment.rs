use bevy::prelude::*;

// ==================================================================
// Helmet

#[derive(Component, Reflect)]
pub enum Helmet {
    None,
    NormalHelmet(NormalHelmet),
}

#[derive(Reflect)]
pub struct NormalHelmet {
    pub armor: f32,
}

pub trait Armor {
    fn armor(&self) -> f32;
}

impl Armor for Helmet {
    fn armor(&self) -> f32 {
        match self {
            Helmet::None => 0.,
            Helmet::NormalHelmet(helmet) => helmet.armor,
        }
    }
}
