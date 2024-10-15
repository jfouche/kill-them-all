mod body_armour;
mod boots;
mod helmet;

pub use body_armour::*;
pub use boots::*;
pub use helmet::*;

use super::rng_provider::{Generator, RngKindProvider};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

// ==================================================================
// EquipmentAssets

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

// ==================================================================
// Affixes traits

pub trait ProvideArmour {
    fn armour(&self) -> f32;
}

pub trait ProvideMoreLife {
    fn more_life(&self) -> f32;
}
pub trait ProvideIncreaseMaxLife {
    fn increase_max_life(&self) -> f32;
}

pub trait ProvideLifeRegen {
    fn life_regen(&self) -> f32;
}
pub trait ProvideIncreaseMovementSpeed {
    fn increase_movement_speed(&self) -> f32;
}

pub trait ProvideIncreaseAttackSpeed {
    fn increase_attack_speed(&self) -> f32;
}

pub trait ProvidePierceChance {
    fn pierce_chance(&self) -> f32;
}

// ==================================================================
// Equipments

#[derive(Component, Default, Reflect)]
pub struct Equipments {
    pub helmet: Helmet,
    pub body_armour: BodyArmour,
    pub boots: Boots,
}

impl ProvideArmour for Equipments {
    fn armour(&self) -> f32 {
        self.helmet.armour() + self.body_armour.armour() + self.boots.armour()
    }
}

impl ProvideMoreLife for Equipments {
    fn more_life(&self) -> f32 {
        self.helmet.more_life() + self.body_armour.more_life() + self.boots.more_life()
    }
}

impl ProvideIncreaseMaxLife for Equipments {
    fn increase_max_life(&self) -> f32 {
        self.helmet.increase_max_life()
            + self.body_armour.increase_max_life()
            + self.boots.increase_max_life()
    }
}

impl ProvideLifeRegen for Equipments {
    fn life_regen(&self) -> f32 {
        self.helmet.life_regen() + self.body_armour.life_regen() + self.boots.life_regen()
    }
}

impl ProvideIncreaseMovementSpeed for Equipments {
    fn increase_movement_speed(&self) -> f32 {
        self.helmet.increase_movement_speed()
            + self.body_armour.increase_movement_speed()
            + self.boots.increase_movement_speed()
    }
}

impl ProvideIncreaseAttackSpeed for Equipments {
    fn increase_attack_speed(&self) -> f32 {
        self.helmet.increase_attack_speed()
            + self.body_armour.increase_attack_speed()
            + self.boots.increase_attack_speed()
    }
}

impl ProvidePierceChance for Equipments {
    fn pierce_chance(&self) -> f32 {
        self.helmet.pierce_chance() + self.body_armour.pierce_chance() + self.boots.pierce_chance()
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

#[derive(Component, Reflect)]
pub enum Equipment {
    Helmet(Helmet),
    BodyArmour(BodyArmour),
    Boots(Boots),
}

impl std::fmt::Display for Equipment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Equipment::Helmet(helmet) => helmet.fmt(f),
            Equipment::BodyArmour(body_armour) => body_armour.fmt(f),
            Equipment::Boots(boots) => boots.fmt(f),
        }
    }
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
