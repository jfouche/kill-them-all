use super::{
    item::{Item, ItemEntityInfo, ItemInfo, ItemLevel, ItemRarity},
    rng_provider::RngKindProvider,
};
use crate::components::inventory::{
    InventoryChanged, PlayerEquipmentChanged, RemoveFromInventoryCommand,
};
use bevy::prelude::*;
use rand::rngs::ThreadRng;
use std::marker::PhantomData;

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

pub struct OrbProvider(RngKindProvider<Orb>);

impl OrbProvider {
    pub fn new() -> Self {
        let mut provider = RngKindProvider::default();
        provider.add(Orb::Transmutation, 4440);
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
    fn reset(item: &mut EntityWorldMut);
    fn gen_affixes(
        item: &mut EntityWorldMut,
        ilevel: ItemLevel,
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

/// Command to change an [ItemRarity::Normal] item to a [ItemRarity::Magic] one
pub struct TransmutationCommand<T> {
    item: Entity,
    orb: Entity,
    _data: PhantomData<T>,
}

impl<T> TransmutationCommand<T> {
    pub fn new(item: Entity, orb: Entity) -> Self {
        TransmutationCommand {
            item,
            orb,
            _data: Default::default(),
        }
    }
}

impl<T> Command for TransmutationCommand<T>
where
    T: OrbAction + Send + 'static,
{
    fn apply(self, world: &mut World) {
        error!("TransmutationCommand::apply()");
        let Some(&Orb::Transmutation) = world.entity(self.orb).get::<Orb>() else {
            error!("Orb is not Orb::Transmutation");
            return;
        };
        let mut item = world.entity_mut(self.item);

        let Some(ilevel) = item.get::<ItemLevel>().copied() else {
            error!("Item doesn't contain ItemLevel");
            return;
        };

        {
            let Some(mut rarity) = item.get_mut::<ItemRarity>() else {
                error!("Item doesn't contain ItemRarity");
                return;
            };
            if *rarity != ItemRarity::Normal {
                error!("Item is not ItemRarity::Normal");
                return;
            }
            *rarity = ItemRarity::Magic;
        }

        let mut rng = rand::rng();

        T::reset(&mut item);
        T::gen_affixes(&mut item, ilevel, ItemRarity::Magic, &mut rng);

        // Despawn orb
        RemoveFromInventoryCommand(self.orb).apply(world);
        world.entity_mut(self.orb).despawn();

        world.trigger(InventoryChanged);
        world.trigger(PlayerEquipmentChanged);
    }
}

/// Command to change an [ItemRarity::Magic] item to a [ItemRarity::Rare] one
pub struct RegalCommand<T> {
    item: Entity,
    orb: Entity,
    _data: PhantomData<T>,
}

impl<T> RegalCommand<T> {
    pub fn new(item: Entity, orb: Entity) -> Self {
        RegalCommand {
            item,
            orb,
            _data: Default::default(),
        }
    }
}

impl<T> Command for RegalCommand<T>
where
    T: OrbAction + Send + 'static,
{
    fn apply(self, world: &mut World) {
        error!("RegalCommand::apply()");
        let Some(&Orb::Regal) = world.entity(self.orb).get::<Orb>() else {
            error!("Orb is not Orb::Regal");
            return;
        };
        let mut item = world.entity_mut(self.item);

        let Some(ilevel) = item.get::<ItemLevel>().copied() else {
            error!("Item doesn't contain ItemLevel");
            return;
        };

        {
            let Some(mut rarity) = item.get_mut::<ItemRarity>() else {
                error!("Item doesn't contain ItemRarity");
                return;
            };
            if *rarity != ItemRarity::Magic {
                error!("Item is not ItemRarity::Magic");
                return;
            }
            *rarity = ItemRarity::Rare;
        }

        let mut rng = rand::rng();

        todo!();
        // T::reset(&mut item);
        // T::gen_affixes(&mut item, ilevel, ItemRarity::Rare, &mut rng);

        // Despawn orb
        RemoveFromInventoryCommand(self.orb).apply(world);
        world.entity_mut(self.orb).despawn();

        world.trigger(InventoryChanged);
        world.trigger(PlayerEquipmentChanged);
    }
}

/// Command to change an [ItemRarity::Normal] item to a [ItemRarity::Rare] one
pub struct ChaosCommand<T> {
    item: Entity,
    orb: Entity,
    _data: PhantomData<T>,
}

impl<T> ChaosCommand<T> {
    pub fn new(item: Entity, orb: Entity) -> Self {
        ChaosCommand {
            item,
            orb,
            _data: Default::default(),
        }
    }
}

impl<T> Command for ChaosCommand<T>
where
    T: OrbAction + Send + 'static,
{
    fn apply(self, world: &mut World) {
        error!("ChaosCommand::apply()");
        let Some(&Orb::Chaos) = world.entity(self.orb).get::<Orb>() else {
            error!("Orb is not Orb::Chaos");
            return;
        };
        let mut item = world.entity_mut(self.item);

        let Some(ilevel) = item.get::<ItemLevel>().copied() else {
            error!("Item doesn't contain ItemLevel");
            return;
        };

        {
            let Some(mut rarity) = item.get_mut::<ItemRarity>() else {
                error!("Item doesn't contain ItemRarity");
                return;
            };
            if *rarity != ItemRarity::Normal {
                error!("Item is not ItemRarity::Normal");
                return;
            }
            *rarity = ItemRarity::Rare;
        }

        let mut rng = rand::rng();

        T::reset(&mut item);
        T::gen_affixes(&mut item, ilevel, ItemRarity::Rare, &mut rng);

        // Despawn orb
        RemoveFromInventoryCommand(self.orb).apply(world);
        world.entity_mut(self.orb).despawn();

        world.trigger(InventoryChanged);
        world.trigger(PlayerEquipmentChanged);
    }
}
