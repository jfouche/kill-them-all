use bevy::prelude::*;

use super::{equipment::Equipment, item::DroppedItem, player::Player};

///
/// The [Inventory] contains all items that carry the [Player] as children
///
#[derive(Component, Default, Reflect)]
#[require(Name(|| Name::new("Inventory")))]
pub struct Inventory([Option<Entity>; Inventory::len()]);

impl Inventory {
    pub const N_COLS: u16 = 8;
    pub const N_ROWS: u16 = 4;

    pub const fn len() -> usize {
        (Inventory::N_COLS * Inventory::N_ROWS) as usize
    }

    fn add(&mut self, item: Entity) -> bool {
        if self.0.iter().any(|o| *o == Some(item)) {
            warn!("Item {item} already in inventory");
            return false;
        }
        let Some(index) = self.0.iter().position(|o| o.is_none()) else {
            info!("Can't add item to inventory because it's full");
            return false;
        };
        self.add_at(item, index)
    }

    fn add_at(&mut self, item: Entity, index: usize) -> bool {
        assert!(index < Self::len());
        if self.0[index].is_some() {
            warn!("Can't add item to a non empty location");
            return false;
        }
        info!("Inventory added {item} at {index}");
        self.0[index] = Some(item);
        true
    }

    fn remove(&mut self, item: Entity) -> bool {
        match self.0.iter().position(|o| *o == Some(item)) {
            Some(index) => {
                self.0[index] = None;
                true
            }
            None => false,
        }
    }

    pub fn at(&self, index: usize) -> Option<Entity> {
        *self.0.get(index)?
    }

    pub fn pos(index: usize) -> InventoryPos {
        assert!(index < Inventory::len());
        InventoryPos {
            col: (index as u16 % Self::N_COLS) as i16,
            row: (index as u16 / Self::N_COLS) as i16,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Reflect)]
pub struct InventoryPos {
    pub col: i16,
    pub row: i16,
}

/// Event to indicate The [Inventory] changed
#[derive(Event)]
pub struct InventoryChanged;

/// Event to indicate The [Player] equipments changed
#[derive(Event)]
pub struct PlayerEquipmentChanged;

/// Try to add an item to the [Inventory].
///
/// If it succeed, it will trigger an [InventoryChanged] event.
pub struct AddToInventoryCommand(pub Entity);

impl Command for AddToInventoryCommand {
    fn apply(self, world: &mut World) {
        let (inventory_entity, mut inventory) =
            world.query::<(Entity, &mut Inventory)>().single_mut(world);

        if inventory.add(self.0) {
            world.entity_mut(inventory_entity).add_child(self.0);
            world.trigger(InventoryChanged);
        }
    }
}

/// Try to add an item to the [Inventory] at a given index.
///
/// If it succeed, it will trigger an [InventoryChanged] event.
pub struct AddToInventoryAtIndexCommand {
    pub item: Entity,
    pub index: usize,
}

impl Command for AddToInventoryAtIndexCommand {
    fn apply(self, world: &mut World) {
        let (inventory_entity, mut inventory) =
            world.query::<(Entity, &mut Inventory)>().single_mut(world);

        // Allow to move an item
        inventory.remove(self.item);
        if inventory.add_at(self.item, self.index) {
            world.entity_mut(inventory_entity).add_child(self.item);
            world.trigger(InventoryChanged);
        }
    }
}

/// Try to remove an item to the [Inventory].
///
/// If it succed, it will trigger an [InventoryChanged] event.
pub struct RemoveFromInventoryCommand(pub Entity);

impl Command for RemoveFromInventoryCommand {
    fn apply(self, world: &mut World) {
        let (inventory_entity, mut inventory) =
            world.query::<(Entity, &mut Inventory)>().single_mut(world);

        if inventory.remove(self.0) {
            world
                .entity_mut(inventory_entity)
                .remove::<InventoryPos>()
                .remove_children(&[self.0]);
            world.trigger(InventoryChanged);
        }
    }
}

/// Command to add an [Equipment] to the [Player].
///
/// If the item is in the [Inventory], it will remove it.
/// If the [Player] already have this kind of [Equipment], it will put the old
/// one to the [Inventory]
pub struct EquipItemCommand(pub Entity);

impl Command for EquipItemCommand {
    fn apply(self, world: &mut World) {
        let mut equipments = world.query::<(Entity, &Equipment, &Parent)>();
        let Ok(equipment_to_equip) = equipments.get(world, self.0).map(|(_, eqp, _)| *eqp) else {
            warn!("Can't equip {} as it's not an Equipment", self.0);
            return;
        };

        // Check it the player already have an item of same type
        let player = world.query_filtered::<Entity, With<Player>>().single(world);
        let old_equipment = equipments
            .iter(world)
            // same parent, same type, but different entity
            .filter(|(entity, eqp, parent)| {
                player == ***parent && **eqp == equipment_to_equip && *entity != self.0
            })
            .map(|(e, _eqp, _p)| e)
            // There should be at most 1 equipment
            .next();

        // Manage inventory
        RemoveFromInventoryCommand(self.0).apply(world);
        if let Some(old_equipment) = old_equipment {
            AddToInventoryCommand(old_equipment).apply(world);
        }

        // Add_child will remove the old parent before applying new parenting
        world.entity_mut(player).add_child(self.0);
        world.trigger(PlayerEquipmentChanged);
    }
}

/// Command to drop an item
pub struct DropItemCommand(pub Entity);

impl Command for DropItemCommand {
    fn apply(self, world: &mut World) {
        let player = world.query_filtered::<Entity, With<Player>>().single(world);
        let inventory = world
            .query_filtered::<Entity, With<Inventory>>()
            .single(world);

        enum Change {
            None,
            Player,
            Inventory,
            Other(Entity),
        }

        let change = world
            .query::<&Parent>()
            .get(world, self.0)
            .map(|p| {
                if **p == player {
                    Change::Player
                } else if **p == inventory {
                    Change::Inventory
                } else {
                    Change::Other(**p)
                }
            })
            .unwrap_or(Change::None);

        match change {
            Change::Player => {
                world.entity_mut(player).remove_children(&[self.0]);
            }
            Change::Inventory => {
                RemoveFromInventoryCommand(self.0).apply(world);
            }
            Change::Other(parent) => {
                world.entity_mut(parent).remove_children(&[self.0]);
            }
            Change::None => {}
        }
        world.entity_mut(self.0).despawn();

        if let Change::Player = change {
            world.trigger(PlayerEquipmentChanged);
        }
    }
}

/// Try to add a [DroppedItem] item to the [crate::components::inventory::Inventory].
///
/// If it succed, it will trigger an [InventoryChanged] event.
pub struct TakeDroppedItemCommand(pub Entity);

impl Command for TakeDroppedItemCommand {
    fn apply(self, world: &mut World) {
        let item_entity = self.0;
        let Ok(item) = world
            .query::<&DroppedItem>()
            .get(world, item_entity)
            .cloned()
        else {
            warn!("Can't take item from {item_entity} as it's not a [DroppedItem]");
            return;
        };
        let (inventory_entity, mut inventory) =
            world.query::<(Entity, &mut Inventory)>().single_mut(world);

        if inventory.add(*item) {
            world.entity_mut(inventory_entity).add_child(*item);
            world.entity_mut(item_entity).despawn();
            world.trigger(InventoryChanged);
        }
    }
}
