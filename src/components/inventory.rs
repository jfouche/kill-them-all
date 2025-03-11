use super::{item::DroppedItem, player::PlayerSkills};
use bevy::prelude::*;

///
/// The [Inventory] contains all items that carry the [crate::components::player::Player] as children
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

/// Event to to toggle the inventory window
#[derive(Event)]
pub struct ToggleInventory;

/// Event to indicate The [Inventory] changed
#[derive(Event)]
pub struct InventoryChanged;

/// Event to indicate The [crate::components::player::Player] equipments changed
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
            // remove from skill if it was a skill
            world
                .query::<&mut PlayerSkills>()
                .get_single_mut(world)
                .expect("PlayerSkills")
                .remove(self.0);

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

            // remove from skill if it was a skill
            world
                .query::<&mut PlayerSkills>()
                .get_single_mut(world)
                .expect("PlayerSkills")
                .remove(self.item);

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
