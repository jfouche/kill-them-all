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
        common::EntityInserter,
        item::{
            Item, ItemDescription, ItemDescriptor, ItemLevel, ItemRarity, ItemRarityProvider,
            ItemTileIndex, ItemTitle, ValueAndTier,
        },
        orb::OrbAction,
        rng_provider::RngKindProvider,
    };
    use bevy::prelude::*;
    use rand::rngs::ThreadRng;
    use std::fmt;

    /// Equiment type
    #[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
    #[require(Item, ItemLevel, ItemRarity)]
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
        fn spawn(&self, commands: &mut Commands, ilevel: u16, rng: &mut ThreadRng) -> Entity {
            let rarity = ItemRarityProvider::gen(rng);

            fn spawn<T>(
                mut equipment: T,
                commands: &mut Commands,
                rarity: ItemRarity,
                rng: &mut ThreadRng,
            ) -> Entity
            where
                T: Component + ItemDescriptor + OrbAction,
            {
                let mut ecmds = commands.spawn_empty();
                let entity = ecmds.id();
                equipment.add_affixes(&mut ecmds, rarity.n_affix(), rng);
                let title = equipment.title();
                let description = equipment.description();
                let tile_index = equipment.tile_index(rarity);
                commands.entity(entity).insert((
                    equipment,
                    rarity,
                    ItemTitle(title),
                    ItemDescription(description),
                    ItemTileIndex(tile_index),
                ));
                entity
            }

            match self {
                EquipmentKind::Amulet => spawn(Amulet::new(ilevel), commands, rarity, rng),
                EquipmentKind::BodyArmour => spawn(BodyArmour::new(ilevel), commands, rarity, rng),
                EquipmentKind::Boots => spawn(Boots::new(ilevel), commands, rarity, rng),
                EquipmentKind::Helmet => spawn(Helmet::new(ilevel), commands, rarity, rng),
                EquipmentKind::Wand => spawn(Wand::new(ilevel), commands, rarity, rng),
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

        pub fn spawn(&mut self, commands: &mut Commands, rng: &mut ThreadRng) -> Option<Entity> {
            Some(self.provider.gen(rng)?.spawn(commands, self.ilevel, rng))
        }
    }

    pub struct AffixProvider<K> {
        ilevel: u16,
        provider: RngKindProvider<K>,
        labels: Vec<String>,
    }

    impl<K> AffixProvider<K>
    where
        K: Copy + Eq + std::hash::Hash,
    {
        pub fn new<T>(ilevel: u16, provider: RngKindProvider<K>) -> Self
        where
            T: Component + ItemDescriptor,
        {
            AffixProvider {
                ilevel,
                provider,
                labels: vec![],
            }
        }

        pub fn ilevel(&self) -> u16 {
            self.ilevel
        }

        pub fn reset(&mut self) {
            self.provider.reset();
            self.labels.truncate(1);
        }

        pub fn gen(&mut self, rng: &mut ThreadRng) -> Option<K> {
            self.provider.gen(rng)
        }

        pub fn set<A, E>(&mut self, entity: &mut E, value: ValueAndTier)
        where
            A: Component + fmt::Display + From<u16>,
            E: EntityInserter,
        {
            let affix = A::from(value.0);
            self.labels.push(format!("{affix} (t{})", value.1));
            entity.insert(affix);
        }

        pub fn item_description(&self) -> String {
            self.labels.join("\n")
        }
    }
}
