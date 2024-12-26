mod amulet;
mod body_armour;
mod boots;
mod helmet;
mod wand;
mod weapon;

use std::fmt::Display;

pub use amulet::Amulet;
pub use body_armour::BodyArmour;
pub use boots::Boots;
pub use helmet::Helmet;
pub use wand::Wand;
pub use weapon::*;

use super::rng_provider::RngKindProvider;
use bevy::prelude::*;
use rand::rngs::ThreadRng;

// ==================================================================
// EquipmentAssets

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
            atlas_layout: world.add_asset(TextureAtlasLayout::from_grid(
                UVec2::new(48, 48),
                16,
                22,
                None,
                None,
            )),
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
}

/// The Tile index in the image atlas
#[derive(Component, Clone, Copy, Deref, Reflect)]
pub struct EquipmentTileIndex(pub usize);

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

pub struct EquipmentEntityInfo {
    pub entity: Entity,
    pub info: EquipmentInfo,
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
    pub fn spawn(&self, commands: &mut Commands, rng: &mut ThreadRng) -> EquipmentEntityInfo {
        match self {
            EquipmentKind::Amulet => Amulet::spawn(commands, rng),
            EquipmentKind::BodyArmour => BodyArmour::spawn(commands, rng),
            EquipmentKind::Boots => Boots::spawn(commands, rng),
            EquipmentKind::Helmet => Helmet::spawn(commands, rng),
            EquipmentKind::Wand => Wand::spawn(commands, rng),
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
        provider.add(EquipmentKind::Wand, 40);
        EquipmentProvider(provider)
    }
}

/// Util component to store all equipments informations, e.g. image and affixes

#[derive(Component, Default, Clone, Reflect)]
pub struct EquipmentInfo {
    pub tile_index: usize,
    pub text: String,
}

impl From<&EquipmentInfo> for Text {
    fn from(value: &EquipmentInfo) -> Self {
        Text(value.text.clone())
    }
}

trait EquipmentUI {
    fn title() -> String;
    fn tile_index(rarity: EquipmentRarity) -> usize;
}

/// Helper to insert affix to an equipment
struct AffixesInserter<'a> {
    labels: Vec<String>,
    commands: EntityCommands<'a>,
    tile_index: usize,
    rarity: EquipmentRarity,
}

impl<'a> AffixesInserter<'a> {
    fn spawn<T>(commands: &'a mut Commands, equipment: T, rng: &mut ThreadRng) -> Self
    where
        T: Component + EquipmentUI,
    {
        let rarity = EquipmentRarityProvider::new()
            .gen(rng)
            .expect("At least one rarity");
        let tile_index = T::tile_index(rarity);
        AffixesInserter {
            labels: vec![T::title()],
            commands: commands.spawn((equipment, rarity, EquipmentTileIndex(tile_index))),
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

    fn equipment_entity(mut self) -> EquipmentEntityInfo {
        let equipment_info = EquipmentInfo {
            text: self.labels.join("\n"),
            tile_index: self.tile_index,
        };
        self.commands.insert(equipment_info.clone());
        EquipmentEntityInfo {
            entity: self.commands.id(),
            info: equipment_info,
        }
    }
}
