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
    #[require(ItemInfo, ItemLevel, ItemRarity)]
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
            let rarity = ItemRarityProvider::gen(rng);
            let mut equipment = match self {
                EquipmentKind::Amulet => commands.spawn((Amulet, rarity)),
                EquipmentKind::BodyArmour => commands.spawn((BodyArmour, rarity)),
                EquipmentKind::Boots => commands.spawn((Boots, rarity)),
                EquipmentKind::Helmet => commands.spawn((Helmet, rarity)),
                EquipmentKind::Wand => commands.spawn((Wand, rarity)),
            };
            let entity = equipment.id();
            let affixes_text = match self {
                EquipmentKind::Amulet => {
                    Amulet::generate_affixes(&mut equipment, rarity, ilevel, rng)
                }
                EquipmentKind::BodyArmour => {
                    BodyArmour::generate_affixes(&mut equipment, rarity, ilevel, rng)
                }
                EquipmentKind::Boots => {
                    Boots::generate_affixes(&mut equipment, rarity, ilevel, rng)
                }
                EquipmentKind::Helmet => {
                    Helmet::generate_affixes(&mut equipment, rarity, ilevel, rng)
                }
                EquipmentKind::Wand => Wand::generate_affixes(&mut equipment, rarity, ilevel, rng),
            };
            let info = ItemInfo {
                text: affixes_text,
                tile_index: Amulet::tile_index(rarity),
            };
            equipment.insert(info.clone());
            ItemEntityInfo { entity, info }
        }
    }

    pub struct EquipmentProvider {
        ilevel: u16,
        provider: RngKindProvider<EquipmentKind>,
    }

    impl EquipmentProvider {
        pub fn new(ilevel: u16) -> Self {
            let mut provider = RngKindProvider::default();
            provider.add(EquipmentKind::Amulet, 4440);
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

    pub struct AffixProvider<K> {
        provider: RngKindProvider<K>,
        labels: Vec<String>,
    }

    impl<K> AffixProvider<K>
    where
        K: Copy + Eq + std::hash::Hash,
    {
        pub fn new<T>(ilevel: u16, provider: RngKindProvider<K>) -> Self
        where
            T: Component + EquipmentUI,
        {
            let title = format!("{} ({})", T::title(), ilevel + 1);
            AffixProvider {
                provider,
                labels: vec![title],
            }
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
            self.labels.push(format!("{affix} ({})", value.1));
            entity.insert(affix);
        }

        pub fn item_text(&self) -> String {
            self.labels.join("\n")
        }
    }

    pub trait EntityInserter {
        fn insert<B: Bundle>(&mut self, bundle: B);
    }

    impl EntityInserter for EntityWorldMut<'_> {
        fn insert<B: Bundle>(&mut self, bundle: B) {
            EntityWorldMut::insert(self, bundle);
        }
    }
    impl EntityInserter for EntityCommands<'_> {
        fn insert<B: Bundle>(&mut self, bundle: B) {
            EntityCommands::insert(self, bundle);
        }
    }
}
