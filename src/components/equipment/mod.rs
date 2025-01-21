pub mod amulet;
pub mod body_armour;
pub mod boots;
pub mod helmet;
pub mod wand;
pub mod weapon;

pub use amulet::Amulet;
pub use body_armour::BodyArmour;
pub use boots::Boots;
pub use helmet::Helmet;
pub use wand::Wand;
pub use weapon::Weapon;

use bevy::prelude::*;
use rand::rngs::ThreadRng;
use std::fmt::Display;

use super::{
    item::{ItemEntityInfo, ItemInfo, ItemLevel, ItemRarity, ItemRarityProvider},
    rng_provider::RngKindProvider,
};

// ==================================================================
// EquipmentAssets

pub const ITEM_SIZE: UVec2 = UVec2::new(48, 48);

#[derive(Resource)]
pub struct EquipmentAssets {
    texture: Handle<Image>,
    atlas_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for EquipmentAssets {
    fn from_world(world: &mut World) -> Self {
        EquipmentAssets {
            texture: world.load_asset(
                "items/Kyrise's 16x16 RPG Icon Pack - V1.3/spritesheet/spritesheet_48x48.png",
            ),
            atlas_layout: world
                .add_asset(TextureAtlasLayout::from_grid(ITEM_SIZE, 16, 22, None, None)),
        }
    }
}

impl EquipmentAssets {
    pub fn image(&self) -> Handle<Image> {
        self.texture.clone()
    }

    pub fn texture_atlas(&self, index: usize) -> TextureAtlas {
        TextureAtlas {
            layout: self.atlas_layout.clone(),
            index,
        }
    }

    pub fn image_node(&self, index: usize) -> ImageNode {
        ImageNode::from_atlas_image(self.image(), self.texture_atlas(index))
    }

    pub fn sprite(&self, index: usize) -> Sprite {
        Sprite {
            image: self.image(),
            texture_atlas: Some(self.texture_atlas(index)),
            ..Default::default()
        }
    }
}

/// Equiment type
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum Equipment {
    Helmet,
    BodyArmour,
    Boots,
    Amulet,
    Weapon,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentKind {
    Amulet,
    BodyArmour,
    Boots,
    Helmet,
    Wand,
}

impl EquipmentKind {
    fn spawn(&self, commands: &mut Commands, ilevel: u16, rng: &mut ThreadRng) -> ItemEntityInfo {
        match self {
            EquipmentKind::Amulet => Amulet::spawn(commands, ilevel, rng),
            EquipmentKind::BodyArmour => BodyArmour::spawn(commands, ilevel, rng),
            EquipmentKind::Boots => Boots::spawn(commands, ilevel, rng),
            EquipmentKind::Helmet => Helmet::spawn(commands, ilevel, rng),
            EquipmentKind::Wand => Wand::spawn(commands, ilevel, rng),
        }
    }
}

pub struct EquipmentProvider {
    ilevel: u16,
    provider: RngKindProvider<EquipmentKind>,
}

impl EquipmentProvider {
    pub fn new(ilevel: u16) -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(EquipmentKind::Amulet, 40);
        provider.add(EquipmentKind::BodyArmour, 40);
        provider.add(EquipmentKind::Boots, 40);
        provider.add(EquipmentKind::Helmet, 40);
        provider.add(EquipmentKind::Wand, 40);
        EquipmentProvider { ilevel, provider }
    }

    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        rng: &mut ThreadRng,
    ) -> Option<ItemEntityInfo> {
        Some(self.provider.gen(rng)?.spawn(commands, self.ilevel, rng))
    }
}

trait EquipmentUI {
    fn title() -> String;
    fn tile_index(rarity: ItemRarity) -> usize;
}

/// Helper to insert affix to an equipment
struct AffixesInserter<'a> {
    labels: Vec<String>,
    commands: EntityCommands<'a>,
    tile_index: usize,
    rarity: ItemRarity,
}

impl<'a> AffixesInserter<'a> {
    fn spawn<T>(commands: &'a mut Commands, equipment: T, ilevel: u16, rng: &mut ThreadRng) -> Self
    where
        T: Component + EquipmentUI,
    {
        let rarity = ItemRarityProvider::new()
            .gen(rng)
            .expect("At least one rarity");
        let tile_index = T::tile_index(rarity);
        AffixesInserter {
            labels: vec![T::title()],
            commands: commands.spawn((equipment, ItemLevel(ilevel), rarity)),
            tile_index,
            rarity,
        }
    }

    fn n_affix(&self) -> u16 {
        self.rarity.n_affix()
    }

    fn insert<A, V>(&mut self, value: V)
    where
        A: Component + Display + From<V>,
    {
        let affix = A::from(value);
        self.labels.push(affix.to_string());
        self.commands.insert(affix);
    }

    fn equipment_entity(mut self) -> ItemEntityInfo {
        let equipment_info = ItemInfo {
            text: self.labels.join("\n"),
            tile_index: self.tile_index,
        };
        self.commands.insert(equipment_info.clone());
        ItemEntityInfo {
            entity: self.commands.id(),
            info: equipment_info,
        }
    }
}
