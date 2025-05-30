use crate::components::{
    equipment::{Amulet, BodyArmour, Boots, Equipment, Helmet, Wand},
    inventory::RemoveFromInventoryEvent,
    item::{update_item_info, ItemDescriptor, ItemRarity, UpdateItemInfo},
    orb::{ActivateOrbEvent, Orb, OrbAction},
};
use bevy::{ecs::component::Mutable, prelude::*};
use std::marker::PhantomData;

pub struct OrbPlugin;

impl Plugin for OrbPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(update_item_info::<Orb>())
            .add_observer(on_activate_orb)
            .add_observer(on_transmute::<Amulet>)
            .add_observer(on_transmute::<Boots>)
            .add_observer(on_transmute::<Helmet>)
            .add_observer(on_transmute::<BodyArmour>)
            .add_observer(on_alteration::<Wand>)
            .add_observer(on_alteration::<Amulet>)
            .add_observer(on_alteration::<Boots>)
            .add_observer(on_alteration::<Helmet>)
            .add_observer(on_alteration::<BodyArmour>)
            .add_observer(on_alteration::<Wand>)
            .add_observer(on_regal::<Amulet>)
            .add_observer(on_regal::<Boots>)
            .add_observer(on_regal::<Helmet>)
            .add_observer(on_regal::<BodyArmour>)
            .add_observer(on_regal::<Wand>)
            .add_observer(on_chaos::<Amulet>)
            .add_observer(on_chaos::<Boots>)
            .add_observer(on_chaos::<Helmet>)
            .add_observer(on_chaos::<BodyArmour>)
            .add_observer(on_chaos::<Wand>);
    }
}

#[derive(Event)]
struct TransmuteEvent<T> {
    orb: Entity,
    item: Entity,
    _phantom: PhantomData<T>,
}

impl<T> TransmuteEvent<T> {
    fn new(orb: Entity, item: Entity) -> Self {
        TransmuteEvent {
            orb,
            item,
            _phantom: PhantomData,
        }
    }
}

#[derive(Event)]
struct AlterationEvent<T> {
    orb: Entity,
    item: Entity,
    _phantom: PhantomData<T>,
}

impl<T> AlterationEvent<T> {
    fn new(orb: Entity, item: Entity) -> Self {
        AlterationEvent {
            orb,
            item,
            _phantom: PhantomData,
        }
    }
}

#[derive(Event)]
struct RegalEvent<T> {
    orb: Entity,
    item: Entity,
    _phantom: PhantomData<T>,
}

impl<T> RegalEvent<T> {
    fn new(orb: Entity, item: Entity) -> Self {
        RegalEvent {
            orb,
            item,
            _phantom: PhantomData,
        }
    }
}
#[derive(Event)]
struct ChaosEvent<T> {
    orb: Entity,
    item: Entity,
    _phantom: PhantomData<T>,
}

impl<T> ChaosEvent<T> {
    fn new(orb: Entity, item: Entity) -> Self {
        ChaosEvent {
            orb,
            item,
            _phantom: PhantomData,
        }
    }
}

fn on_activate_orb(
    trigger: Trigger<ActivateOrbEvent>,
    mut commands: Commands,
    orbs: Query<&Orb>,
    equipments: Query<&Equipment>,
) {
    let orb_entity = trigger.orb;
    let Ok(&orb) = orbs.get(orb_entity) else {
        warn!("Can't apply orb as {orb_entity} is not an Orb");
        return;
    };

    let item_entity = trigger.item;
    let Ok(&equipment) = equipments.get(item_entity) else {
        warn!("Item is not an Equipment");
        return;
    };

    match orb {
        Orb::Transmutation => match equipment {
            Equipment::Amulet => {
                commands.trigger(TransmuteEvent::<Amulet>::new(orb_entity, item_entity));
            }
            Equipment::BodyArmour => {
                commands.trigger(TransmuteEvent::<BodyArmour>::new(orb_entity, item_entity));
            }
            Equipment::Boots => {
                commands.trigger(TransmuteEvent::<Boots>::new(orb_entity, item_entity));
            }
            Equipment::Helmet => {
                commands.trigger(TransmuteEvent::<Helmet>::new(orb_entity, item_entity));
            }
            Equipment::Weapon => {
                // TODO: use Weapon
                commands.trigger(TransmuteEvent::<Wand>::new(orb_entity, item_entity));
            }
        },
        Orb::Alteration => match equipment {
            Equipment::Amulet => {
                commands.trigger(AlterationEvent::<Amulet>::new(orb_entity, item_entity));
            }
            Equipment::BodyArmour => {
                commands.trigger(AlterationEvent::<BodyArmour>::new(orb_entity, item_entity));
            }
            Equipment::Boots => {
                commands.trigger(AlterationEvent::<Boots>::new(orb_entity, item_entity));
            }
            Equipment::Helmet => {
                commands.trigger(AlterationEvent::<Helmet>::new(orb_entity, item_entity));
            }
            Equipment::Weapon => {
                // TODO: use Weapon
                commands.trigger(AlterationEvent::<Wand>::new(orb_entity, item_entity));
            }
        },
        Orb::Regal => match equipment {
            Equipment::Amulet => {
                commands.trigger(RegalEvent::<Amulet>::new(orb_entity, item_entity));
            }
            Equipment::BodyArmour => {
                commands.trigger(RegalEvent::<BodyArmour>::new(orb_entity, item_entity));
            }
            Equipment::Boots => {
                commands.trigger(RegalEvent::<Boots>::new(orb_entity, item_entity));
            }
            Equipment::Helmet => {
                commands.trigger(RegalEvent::<Helmet>::new(orb_entity, item_entity));
            }
            Equipment::Weapon => {
                // TODO: use Weapon
                commands.trigger(RegalEvent::<Wand>::new(orb_entity, item_entity));
            }
        },
        Orb::Chaos => match equipment {
            Equipment::Amulet => {
                commands.trigger(ChaosEvent::<Amulet>::new(orb_entity, item_entity));
            }
            Equipment::BodyArmour => {
                commands.trigger(ChaosEvent::<BodyArmour>::new(orb_entity, item_entity));
            }
            Equipment::Boots => {
                commands.trigger(ChaosEvent::<Boots>::new(orb_entity, item_entity));
            }
            Equipment::Helmet => {
                commands.trigger(ChaosEvent::<Helmet>::new(orb_entity, item_entity));
            }
            Equipment::Weapon => {
                // TODO: use Weapon
                commands.trigger(ChaosEvent::<Wand>::new(orb_entity, item_entity));
            }
        },
    }
}

fn on_transmute<T>(
    trigger: Trigger<TransmuteEvent<T>>,
    mut commands: Commands,
    orbs: Query<&Orb>,
    mut items: Query<(&mut T, &mut ItemRarity)>,
) where
    T: Component<Mutability = Mutable> + ItemDescriptor + OrbAction,
{
    let Ok(&Orb::Transmutation) = orbs.get(trigger.orb) else {
        error!("on_transmute: Orb is not Orb::Transmutation");
        return;
    };

    let Ok((mut item, mut rarity)) = items.get_mut(trigger.item) else {
        error!("on_transmute: Can't transmute a NON Item");
        return;
    };

    if *rarity != ItemRarity::Normal {
        error!("on_transmute: Item is not ItemRarity::Normal");
        return;
    }

    info!("Applying Transmutation on {}", trigger.item);

    let mut rng = rand::rng();
    let mut item_cmds = commands.entity(trigger.item);
    item.reset_affixes(&mut item_cmds);
    *rarity = ItemRarity::Magic;
    item.add_affixes(&mut item_cmds, rarity.n_affix(), &mut rng);
    commands.queue(UpdateItemInfo::<T>::new(trigger.item));

    // Despawn orb
    commands.trigger(RemoveFromInventoryEvent(trigger.orb));
    commands.entity(trigger.orb).despawn();
}

fn on_alteration<T>(
    trigger: Trigger<AlterationEvent<T>>,
    mut commands: Commands,
    orbs: Query<&Orb>,
    mut items: Query<(&mut T, &ItemRarity)>,
) where
    T: Component<Mutability = Mutable> + ItemDescriptor + OrbAction,
{
    let Ok(&Orb::Alteration) = orbs.get(trigger.orb) else {
        error!("on_alteration: Orb is not Orb::Alteration");
        return;
    };

    let Ok((mut item, rarity)) = items.get_mut(trigger.item) else {
        error!("on_alteration: Can't transmute a NON Item");
        return;
    };

    if *rarity != ItemRarity::Magic {
        error!("on_transmute: Item is not ItemRarity::Magic");
        return;
    }

    info!("Applying Alteration on {}", trigger.item);

    let mut rng = rand::rng();
    let mut item_cmds = commands.entity(trigger.item);
    item.reset_affixes(&mut item_cmds);
    item.add_affixes(&mut item_cmds, rarity.n_affix(), &mut rng);
    commands.queue(UpdateItemInfo::<T>::new(trigger.item));

    // Despawn orb
    commands.trigger(RemoveFromInventoryEvent(trigger.orb));
    commands.entity(trigger.orb).despawn();
}

fn on_regal<T>(
    trigger: Trigger<RegalEvent<T>>,
    mut commands: Commands,
    orbs: Query<&Orb>,
    mut items: Query<(&mut T, &mut ItemRarity)>,
) where
    T: Component<Mutability = Mutable> + ItemDescriptor + OrbAction,
{
    let Ok(&Orb::Regal) = orbs.get(trigger.orb) else {
        error!("on_regal: Orb is not Orb::Regal");
        return;
    };

    let Ok((mut item, mut rarity)) = items.get_mut(trigger.item) else {
        error!("on_regal: Can't regal a NON Item");
        return;
    };

    if *rarity != ItemRarity::Magic {
        error!("on_regal: Item is not ItemRarity::Magic");
        return;
    }

    info!("Applying Regal on {}", trigger.item);

    let mut rng = rand::rng();
    let mut item_cmds = commands.entity(trigger.item);
    *rarity = ItemRarity::Rare;
    item.add_affixes(&mut item_cmds, 1, &mut rng);
    commands.queue(UpdateItemInfo::<T>::new(trigger.item));

    // Despawn orb
    commands.trigger(RemoveFromInventoryEvent(trigger.orb));
    commands.entity(trigger.orb).despawn();
}

fn on_chaos<T>(
    trigger: Trigger<ChaosEvent<T>>,
    mut commands: Commands,
    orbs: Query<&Orb>,
    mut items: Query<(&mut T, &ItemRarity)>,
) where
    T: Component<Mutability = Mutable> + ItemDescriptor + OrbAction,
{
    let Ok(&Orb::Chaos) = orbs.get(trigger.orb) else {
        error!("on_chaos: Orb is not Orb::Chaos");
        return;
    };

    let Ok((mut item, rarity)) = items.get_mut(trigger.item) else {
        error!("on_chaos: Can't transmute a NON Item");
        return;
    };

    if *rarity != ItemRarity::Rare {
        error!("on_chaos: Item is not ItemRarity::Rare");
        return;
    }

    info!("Applying Chaos on {}", trigger.item);

    let mut rng = rand::rng();
    let mut item_cmds = commands.entity(trigger.item);
    item.reset_affixes(&mut item_cmds);
    item.add_affixes(&mut item_cmds, ItemRarity::Rare.n_affix(), &mut rng);
    commands.queue(UpdateItemInfo::<T>::new(trigger.item));

    // Despawn orb
    commands.trigger(RemoveFromInventoryEvent(trigger.orb));
    commands.entity(trigger.orb).despawn();
}
