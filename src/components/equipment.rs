use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use super::rng_provider::{Generator, RngKindProvider};

// ==================================================================
// InventoryAssets

#[derive(Resource)]
pub struct EquipmentAssets {
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
}

impl EquipmentAssets {
    pub fn load(
        asset_server: &AssetServer,
        mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let texture_atlas_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
            UVec2::new(48, 48),
            16,
            22,
            None,
            None,
        ));

        let texture = asset_server
            .load("items/Kyrise's 16x16 RPG Icon Pack - V1.3/spritesheet/spritesheet_48x48.png");

        EquipmentAssets {
            texture,
            texture_atlas_layout,
        }
    }

    pub fn image(&self, equipment: &Equipment) -> (Handle<Image>, TextureAtlas) {
        match equipment {
            Equipment::Helmet(helmet) => self.helmet(helmet),
            Equipment::BodyArmour(body_armour) => self.body_armour(body_armour),
            Equipment::Boots(boots) => self.boots(boots),
        }
    }

    pub fn helmet(&self, helmet: &Helmet) -> (Handle<Image>, TextureAtlas) {
        let index = match helmet {
            Helmet::None => 351,
            Helmet::Normal(_) => 182,
            Helmet::Magic(_) => 184,
        };
        (
            self.texture.clone(),
            TextureAtlas {
                layout: self.texture_atlas_layout.clone(),
                index,
            },
        )
    }

    pub fn body_armour(&self, body_armour: &BodyArmour) -> (Handle<Image>, TextureAtlas) {
        let index = match body_armour {
            BodyArmour::None => 351,
            BodyArmour::Normal(_) => 0,
            BodyArmour::Magic(_) => 2,
        };
        (
            self.texture.clone(),
            TextureAtlas {
                layout: self.texture_atlas_layout.clone(),
                index,
            },
        )
    }

    pub fn boots(&self, boots: &Boots) -> (Handle<Image>, TextureAtlas) {
        let index = match boots {
            Boots::None => 351,
            Boots::Normal(_) => 63,
            Boots::Magic(_) => 65,
        };
        (
            self.texture.clone(),
            TextureAtlas {
                layout: self.texture_atlas_layout.clone(),
                index,
            },
        )
    }
}

pub trait Armor {
    fn armor(&self) -> f32;
}

// ==================================================================
// Helmet

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
    fn generate(rng: &mut ThreadRng) -> Self {
        NormalHelmet {
            armor: rng.gen_range(1..=2) as f32,
        }
    }
}

#[derive(Copy, Clone, Reflect)]
pub struct MagicHelmet {
    pub base: NormalHelmet,
    pub life: f32,
}

impl MagicHelmet {
    fn generate(rng: &mut ThreadRng) -> Self {
        MagicHelmet {
            base: NormalHelmet::generate(rng),
            life: rng.gen_range(5..=10) as f32,
        }
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

// ==================================================================
// BodyArmour

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
    fn generate(rng: &mut ThreadRng) -> Self {
        NormalBodyArmour {
            armor: rng.gen_range(1..=2) as f32,
        }
    }
}

#[derive(Copy, Clone, Reflect)]
pub struct MagicBodyArmour {
    pub base: NormalBodyArmour,
    pub life: f32,
}

impl MagicBodyArmour {
    fn generate(rng: &mut ThreadRng) -> Self {
        MagicBodyArmour {
            base: NormalBodyArmour::generate(rng),
            life: rng.gen_range(5..=10) as f32,
        }
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

// ==================================================================
// Boot

#[derive(Component, Clone, Reflect)]
pub enum Boots {
    None,
    Normal(NormalBoots),
    Magic(MagicBoots),
}

#[derive(Copy, Clone, Reflect)]
pub struct NormalBoots {
    pub armor: f32,
}

impl NormalBoots {
    fn generate(rng: &mut ThreadRng) -> Self {
        NormalBoots {
            armor: rng.gen_range(1..=2) as f32,
        }
    }
}

#[derive(Copy, Clone, Reflect)]
pub struct MagicBoots {
    pub base: NormalBoots,
    pub life: f32,
}

impl MagicBoots {
    fn generate(rng: &mut ThreadRng) -> Self {
        MagicBoots {
            base: NormalBoots::generate(rng),
            life: rng.gen_range(5..=10) as f32,
        }
    }
}

impl Armor for Boots {
    fn armor(&self) -> f32 {
        match self {
            Boots::None => 0.,
            Boots::Normal(boot) => boot.armor,
            Boots::Magic(boot) => boot.base.armor,
        }
    }
}

// ==================================================================
// EquipmentProvider

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentKind {
    NormalHelmet,
    MagicHelmet,
    NormalBodyArmour,
    MagicBodyArmour,
    NormalBoots,
    MagicBoots,
}

impl Generator<Equipment> for EquipmentKind {
    fn generate(&self, rng: &mut ThreadRng) -> Equipment {
        match self {
            EquipmentKind::NormalHelmet => {
                Equipment::Helmet(Helmet::Normal(NormalHelmet::generate(rng)))
            }
            EquipmentKind::MagicHelmet => {
                Equipment::Helmet(Helmet::Magic(MagicHelmet::generate(rng)))
            }
            EquipmentKind::NormalBodyArmour => {
                Equipment::BodyArmour(BodyArmour::Normal(NormalBodyArmour::generate(rng)))
            }
            EquipmentKind::MagicBodyArmour => {
                Equipment::BodyArmour(BodyArmour::Magic(MagicBodyArmour::generate(rng)))
            }
            EquipmentKind::NormalBoots => {
                Equipment::Boots(Boots::Normal(NormalBoots::generate(rng)))
            }
            EquipmentKind::MagicBoots => Equipment::Boots(Boots::Magic(MagicBoots::generate(rng))),
        }
    }
}

#[derive(Component)]
pub enum Equipment {
    Helmet(Helmet),
    BodyArmour(BodyArmour),
    Boots(Boots),
}

#[derive(Deref, DerefMut)]
pub struct EquipmentProvider(RngKindProvider<EquipmentKind, Equipment>);

impl EquipmentProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::<EquipmentKind, Equipment>::default();
        provider.add(EquipmentKind::NormalHelmet, 40);
        provider.add(EquipmentKind::MagicHelmet, 20);
        provider.add(EquipmentKind::NormalBodyArmour, 40);
        provider.add(EquipmentKind::MagicBodyArmour, 20);
        provider.add(EquipmentKind::NormalBoots, 40);
        provider.add(EquipmentKind::MagicBoots, 20);

        EquipmentProvider(provider)
    }
}
