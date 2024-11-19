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
pub struct TileIndex(usize);

/// Equipment Rarity
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentRarityKind {
    Normal,
    Magic,
    Rare,
}

impl EquipmentRarityKind {
    pub fn n_affix(&self) -> u16 {
        match self {
            EquipmentRarityKind::Normal => 0,
            EquipmentRarityKind::Magic => 1,
            EquipmentRarityKind::Rare => 2,
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct EquipmentRarityProvider(RngKindProvider<EquipmentRarityKind>);

impl EquipmentRarityProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(EquipmentRarityKind::Normal, 10);
        provider.add(EquipmentRarityKind::Magic, 8);
        provider.add(EquipmentRarityKind::Rare, 5);
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
