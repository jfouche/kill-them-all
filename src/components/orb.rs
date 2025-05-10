use super::{
    item::{Item, ItemEntityInfo, ItemInfo, ItemRarity},
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

/// Orb item
#[derive(Component, Clone, Copy, Eq, PartialEq, Hash)]
#[require(Item)]
pub enum Orb {
    /// Transform a normal item to a magic one
    Transmutation,
    /// Transform a magic item to a new magic one
    Alteration,
    /// Transform a magic item to a rare one
    Regal,
    /// Transform a rare item to a new rare one, keeping the same base
    Chaos,
}

impl From<Orb> for ItemInfo {
    fn from(orb: Orb) -> Self {
        match orb {
            Orb::Transmutation => ItemInfo {
                tile_index: 153,
                text: "Transform a normal item to a magic one".into(),
            },
            Orb::Alteration => ItemInfo {
                tile_index: 151,
                text: "Transform a magic item to a new magic one".into(),
            },
            Orb::Regal => ItemInfo {
                tile_index: 155,
                text: "Transform a magic item to a rare one".into(),
            },
            Orb::Chaos => ItemInfo {
                tile_index: 150,
                text: "Transform a rare item to a new rare one, keeping the same base".into(),
            },
        }
    }
}

/// Tool to provide random orb
pub struct OrbProvider;

impl OrbProvider {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> ItemEntityInfo {
        let mut provider = RngKindProvider::default();
        provider.add(Orb::Transmutation, 40);
        provider.add(Orb::Alteration, 40);
        provider.add(Orb::Regal, 40);
        provider.add(Orb::Chaos, 40);

        let orb = provider.gen(rng).expect("At least 1 orb");
        let info: ItemInfo = orb.into();
        let entity = commands.spawn((orb, info.clone())).id();
        ItemEntityInfo { entity, info }
    }
}

pub trait OrbAction {
    fn affix_text(&self) -> String;

    fn affix_reset(&mut self, ecommands: &mut EntityCommands);

    fn affix_gen(
        &mut self,
        ecommands: &mut EntityCommands,
        count: u16,
        rarity: ItemRarity,
        rng: &mut ThreadRng,
    );
}

/// Event to activate an [Orb] on an [Item]
#[derive(Event)]
pub struct ActivateOrbEvent {
    pub orb: Entity,
    pub item: Entity,
}
