pub mod amulet;
pub mod body_armour;
pub mod boots;
pub mod helmet;
pub mod wand;
pub mod weapon;

pub use amulet::Amulet;
pub use body_armour::BodyArmour;
pub use boots::Boots;
pub use common::{Equipment, EquipmentProvider};
pub use helmet::Helmet;
pub use wand::Wand;
pub use weapon::Weapon;

mod common {

    use super::*;
    use crate::components::{
        item::{ItemEntityInfo, ItemInfo, ItemLevel, ItemRarity, ItemRarityProvider, ValueAndTier},
        rng_provider::RngKindProvider,
    };
    use bevy::prelude::*;
    use rand::rngs::ThreadRng;
    use std::fmt;

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
        fn spawn(
            &self,
            commands: &mut Commands,
            ilevel: u16,
            rng: &mut ThreadRng,
        ) -> ItemEntityInfo {
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

    pub trait EquipmentUI {
        fn title() -> String;
        fn tile_index(rarity: ItemRarity) -> usize;
    }

    /// Helper to insert affix to an equipment
    pub struct AffixesInserter<'a> {
        labels: Vec<String>,
        commands: EntityCommands<'a>,
        tile_index: usize,
        rarity: ItemRarity,
    }

    impl<'a> AffixesInserter<'a> {
        pub fn spawn<T>(
            commands: &'a mut Commands,
            equipment: T,
            ilevel: u16,
            rng: &mut ThreadRng,
        ) -> Self
        where
            T: Component + EquipmentUI,
        {
            let rarity = ItemRarityProvider::new()
                .gen(rng)
                .expect("At least one rarity");
            let tile_index = T::tile_index(rarity);
            let title = format!("{} ({})", T::title(), ilevel + 1);
            AffixesInserter {
                labels: vec![title],
                commands: commands.spawn((equipment, ItemLevel(ilevel), rarity)),
                tile_index,
                rarity,
            }
        }

        pub fn n_affix(&self) -> u16 {
            self.rarity.n_affix()
        }

        pub fn insert<A>(&mut self, value: ValueAndTier)
        where
            A: Component + fmt::Display + From<u16>,
        {
            let affix = A::from(value.0);
            self.labels.push(format!("{affix} ({})", value.1));
            self.commands.insert(affix);
        }

        pub fn equipment_entity(mut self) -> ItemEntityInfo {
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
}
