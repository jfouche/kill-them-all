use super::{
    item::{Item, ItemDescription, ItemDescriptor, ItemRarity, ItemTileIndex, ItemTitle},
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

impl ItemDescriptor for Orb {
    fn title(&self) -> String {
        match self {
            Orb::Transmutation => "Orb of transmutation".into(),
            Orb::Alteration => "Orb of alteration".into(),
            Orb::Regal => "Orb of regal".into(),
            Orb::Chaos => "Orb of chaos".into(),
        }
    }

    fn description(&self) -> String {
        match self {
            Orb::Transmutation => "Transform a normal item to a magic one".into(),
            Orb::Alteration => "Transform a magic item to a new magic one".into(),
            Orb::Regal => "Transform a magic item to a rare one".into(),
            Orb::Chaos => "Transform a rare item to a new rare one, keeping the same base".into(),
        }
    }

    fn tile_index(&self, _rarity: ItemRarity) -> usize {
        match self {
            Orb::Transmutation => 153,
            Orb::Alteration => 151,
            Orb::Regal => 155,
            Orb::Chaos => 150,
        }
    }
}

/// Tool to provide random orb
pub struct OrbProvider;

impl OrbProvider {
    pub fn spawn(commands: &mut Commands, rng: &mut ThreadRng) -> Entity {
        let mut provider = RngKindProvider::default();
        provider.add(Orb::Transmutation, 40);
        provider.add(Orb::Alteration, 40);
        provider.add(Orb::Regal, 40);
        provider.add(Orb::Chaos, 40);

        let orb = provider.gen(rng).expect("At least 1 orb");
        let title = orb.title();
        let description = orb.description();
        let tile_index = orb.tile_index(ItemRarity::Normal);
        commands
            .spawn((
                orb,
                ItemTitle(title),
                ItemDescription(description),
                ItemTileIndex(tile_index),
            ))
            .id()
    }
}

pub trait OrbAction {
    fn reset_affixes(&mut self, ecommands: &mut EntityCommands);

    /// Add `count` affixes to an [Item]
    fn add_affixes(&mut self, ecommands: &mut EntityCommands, count: u16, rng: &mut ThreadRng);
}

/// Event to activate an [Orb] on an [Item]
#[derive(Event)]
pub struct ActivateOrbEvent {
    pub orb: Entity,
    pub item: Entity,
}
