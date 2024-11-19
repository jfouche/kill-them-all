mod amulet;
mod body_armour;
mod boots;
mod helmet;

pub use amulet::*;
pub use body_armour::*;
pub use boots::*;
pub use helmet::*;

use super::rng_provider::RngKindProvider;
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

    pub fn texture(&self) -> Handle<Image> {
        self.texture.clone()
    }

    pub fn atlas(&self, index: usize) -> TextureAtlas {
        TextureAtlas {
            layout: self.texture_atlas_layout.clone(),
            index,
        }
    }
}

#[derive(Component, Clone, Copy, Deref, Reflect)]
pub struct TileIndex(pub usize);

/// Equipment Rarity
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentRarity {
    Normal,
    Magic,
    Rare,
}

impl EquipmentRarity {
    pub fn n_affix(&self) -> u16 {
        match self {
            EquipmentRarity::Normal => 1,
            EquipmentRarity::Magic => 2,
            EquipmentRarity::Rare => 3,
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct EquipmentRarityProvider(RngKindProvider<EquipmentRarity>);

impl EquipmentRarityProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(EquipmentRarity::Normal, 10);
        provider.add(EquipmentRarity::Magic, 8);
        provider.add(EquipmentRarity::Rare, 5);
        EquipmentRarityProvider(provider)
    }
}

pub struct EquipmentEntity {
    pub entity: Entity,
    pub tile_index: usize,
    pub label: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentKind {
    Amulet,
    BodyArmour,
    Boots,
    Helmet,
}

impl EquipmentKind {
    pub fn spawn(&self, commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntity {
        match self {
            EquipmentKind::Amulet => Amulet::spawn(commands, rng),
            EquipmentKind::BodyArmour => BodyArmour::spawn(commands, rng),
            EquipmentKind::Boots => Boots::spawn(commands, rng),
            EquipmentKind::Helmet => Helmet::spawn(commands, rng),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct EquipmentProvider(RngKindProvider<EquipmentKind>);

impl EquipmentProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(EquipmentKind::Amulet, 40);
        provider.add(EquipmentKind::BodyArmour, 40);
        provider.add(EquipmentKind::Boots, 40);
        provider.add(EquipmentKind::Helmet, 40);
        EquipmentProvider(provider)
    }
}
