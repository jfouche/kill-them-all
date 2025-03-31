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
    /// Transform a magic item to a rare one
    Regal,
    /// Transform a rare item to a new rare one, keeping the same base
    Chaos,
}

impl From<Orb> for ItemInfo {
    fn from(orb: Orb) -> Self {
        match orb {
            Orb::Transmutation => ItemInfo {
                tile_index: 151,
                text: "Transform a normal item to a magic one".into(),
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

// /// Transform a normal item to a magic one
// #[derive(Component, Default)]
// #[require(Orb, Name(|| Name::new("TransmutationOrb")))]
// pub struct TransmutationOrb;

// impl From<TransmutationOrb> for ItemInfo {
//     fn from(_: TransmutationOrb) -> Self {
//         ItemInfo {
//             tile_index: 151,
//             text: "Transform a normal item to a magic one".into(),
//         }
//     }
// }

// /// Transform a magic item to a rare one
// #[derive(Component, Default)]
// #[require(Orb, Name(|| Name::new("RegalOrb")))]
// pub struct RegalOrb;

// impl From<RegalOrb> for ItemInfo {
//     fn from(_: RegalOrb) -> Self {
//         ItemInfo {
//             tile_index: 155,
//             text: "Transform a magic item to a rare one".into(),
//         }
//     }
// }

// /// Transform a rare item to a new rare one, keeping the same base
// #[derive(Component, Default)]
// #[require(Orb, Name(|| Name::new("ChaosOrb")))]
// pub struct ChaosOrb;

// impl From<ChaosOrb> for ItemInfo {
//     fn from(_: ChaosOrb) -> Self {
//         ItemInfo {
//             tile_index: 150,
//             text: "Transform a rare item to a new rare one, keeping the same base".into(),
//         }
//     }
// }

// #[derive(Clone, Copy, Eq, PartialEq, Hash)]
// enum OrbKind {
//     Transmutation,
//     Regal,
//     Chaos,
// }

pub struct OrbProvider(RngKindProvider<Orb>);

impl OrbProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(Orb::Transmutation, 40);
        provider.add(Orb::Regal, 40);
        provider.add(Orb::Chaos, 40);
        OrbProvider(provider)
    }

    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        rng: &mut ThreadRng,
    ) -> Option<ItemEntityInfo> {
        let orb = self.0.gen(rng)?;
        let info: ItemInfo = orb.into();
        let entity = commands.spawn((orb, info.clone())).id();
        Some(ItemEntityInfo { entity, info })
    }
}

pub trait OrbAction {
    fn apply_orb(item: &mut EntityCommands, orb: Orb);
}

/// Event to activate an [Orb] on an [Item]
#[derive(Event)]
pub struct ActivateOrbEvent {
    pub orb: Entity,
    pub item: Entity,
}
