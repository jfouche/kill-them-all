use crate::components::{
    equipment::{Amulet, BodyArmour, Boots, Equipment, Helmet, Wand},
    orb::{ActivateOrbEvent, ChaosCommand, Orb, RegalCommand, TransmutationCommand},
};
use bevy::prelude::*;

pub struct OrbPlugin;

impl Plugin for OrbPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_activate_orb);
    }
}

fn on_activate_orb(
    trigger: Trigger<ActivateOrbEvent>,
    mut commands: Commands,
    orbs: Query<&Orb>,
    equipments: Query<&Equipment>,
) {
    error!("on_activate_orb()");
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
                commands.queue(TransmutationCommand::<Amulet>::new(item_entity, orb_entity));
            }
            Equipment::BodyArmour => {
                commands.queue(TransmutationCommand::<BodyArmour>::new(
                    item_entity,
                    orb_entity,
                ));
            }
            Equipment::Boots => {
                commands.queue(TransmutationCommand::<Boots>::new(item_entity, orb_entity));
            }
            Equipment::Helmet => {
                commands.queue(TransmutationCommand::<Helmet>::new(item_entity, orb_entity));
            }
            Equipment::Weapon => {
                // TODO: use Weapon
                commands.queue(TransmutationCommand::<Wand>::new(item_entity, orb_entity));
            }
        },
        Orb::Regal => match equipment {
            Equipment::Amulet => {
                commands.queue(RegalCommand::<Amulet>::new(item_entity, orb_entity));
            }
            Equipment::BodyArmour => {
                commands.queue(RegalCommand::<BodyArmour>::new(item_entity, orb_entity));
            }
            Equipment::Boots => {
                commands.queue(RegalCommand::<Boots>::new(item_entity, orb_entity));
            }
            Equipment::Helmet => {
                commands.queue(RegalCommand::<Helmet>::new(item_entity, orb_entity));
            }
            Equipment::Weapon => {
                // TODO: use Weapon
                commands.queue(RegalCommand::<Wand>::new(item_entity, orb_entity));
            }
        },
        Orb::Chaos => match equipment {
            Equipment::Amulet => {
                commands.queue(ChaosCommand::<Amulet>::new(item_entity, orb_entity));
            }
            Equipment::BodyArmour => {
                commands.queue(ChaosCommand::<BodyArmour>::new(item_entity, orb_entity));
            }
            Equipment::Boots => {
                commands.queue(ChaosCommand::<Boots>::new(item_entity, orb_entity));
            }
            Equipment::Helmet => {
                commands.queue(ChaosCommand::<Helmet>::new(item_entity, orb_entity));
            }
            Equipment::Weapon => {
                // TODO: use Weapon
                commands.queue(ChaosCommand::<Wand>::new(item_entity, orb_entity));
            }
        },
    }
}
