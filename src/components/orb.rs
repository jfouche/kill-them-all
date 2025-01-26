use super::{
    item::{Item, ItemEntityInfo, ItemInfo},
    rng_provider::RngKindProvider,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;

/// Common component for all orbs
#[derive(Component, Default)]
#[require(Item)]
pub struct Orb;

/// Transform a normal item to a magic one
#[derive(Component, Default)]
#[require(Orb, Name(|| Name::new("TransmutationOrb")))]
pub struct TransmutationOrb;

impl From<TransmutationOrb> for ItemInfo {
    fn from(_: TransmutationOrb) -> Self {
        ItemInfo {
            tile_index: 151,
            text: "Transform a normal item to a magic one".into(),
        }
    }
}

/// Transform a magic item to a rare one
#[derive(Component, Default)]
#[require(Orb, Name(|| Name::new("RegalOrb")))]
pub struct RegalOrb;

impl From<RegalOrb> for ItemInfo {
    fn from(_: RegalOrb) -> Self {
        ItemInfo {
            tile_index: 155,
            text: "Transform a magic item to a rare one".into(),
        }
    }
}

/// Transform a rare item to a new rare one, keeping the same base
#[derive(Component, Default)]
#[require(Orb, Name(|| Name::new("ChaosOrb")))]
pub struct ChaosOrb;

impl From<ChaosOrb> for ItemInfo {
    fn from(_: ChaosOrb) -> Self {
        ItemInfo {
            tile_index: 150,
            text: "Transform a rare item to a new rare one, keeping the same base".into(),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum OrbKind {
    Transmutation,
    Regal,
    Chaos,
}

pub struct OrbProvider(RngKindProvider<OrbKind>);

impl OrbProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(OrbKind::Transmutation, 40);
        provider.add(OrbKind::Regal, 40);
        provider.add(OrbKind::Chaos, 40);
        OrbProvider(provider)
    }

    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        rng: &mut ThreadRng,
    ) -> Option<ItemEntityInfo> {
        match self.0.gen(rng)? {
            OrbKind::Transmutation => Some(Self::spawn_orb::<TransmutationOrb>(commands)),
            OrbKind::Regal => Some(Self::spawn_orb::<RegalOrb>(commands)),
            OrbKind::Chaos => Some(Self::spawn_orb::<ChaosOrb>(commands)),
        }
    }

    fn spawn_orb<T>(commands: &mut Commands) -> ItemEntityInfo
    where
        T: Component + Default + Into<ItemInfo>,
    {
        let info: ItemInfo = T::default().into();
        let entity = commands.spawn((T::default(), info.clone())).id();
        ItemEntityInfo { entity, info }
    }
}
