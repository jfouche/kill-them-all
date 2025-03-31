use bevy::prelude::*;

use crate::components::{
    equipment::{Amulet, Equipment},
    item::{Item, ItemRarity},
    orb::{ActivateOrbEvent, Orb},
};

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
    equipments: Query<(&Equipment, &ItemRarity)>,
) {
    error!("on_activate_orb()");
    let orb_entity = trigger.orb;
    let item_entity = trigger.item;

    let Ok(orb) = orbs.get(orb_entity) else {
        warn!("Can't apply orb as {orb_entity} is not an Orb");
        return;
    };
    match *orb {
        Orb::Transmutation => {
            if let Ok((equipment, &ItemRarity::Normal)) = equipments.get(item_entity) {
                let mut equipment_cmd = commands.entity(item_entity);
                match *equipment {
                    Equipment::Amulet => {
                        Amulet::reset(&mut equipment_cmd);
                    }
                    _ => {
                        todo!("Not implemented");
                    }
                }
            }
        }
        Orb::Regal => {
            todo!("Not implemented");
        }
        Orb::Chaos => {
            todo!("Not implemented");
        }
    }
}
