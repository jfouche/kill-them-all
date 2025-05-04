use bevy::prelude::*;

///
/// The [Inventory] contains all items that carry the [crate::components::player::Player] as children
///
#[derive(Component, Default, Reflect)]
#[require(Name::new("Inventory"))]
pub struct Inventory([Option<Entity>; Inventory::len()]);

impl Inventory {
    pub const N_COLS: u16 = 8;
    pub const N_ROWS: u16 = 4;

    pub const fn len() -> usize {
        (Inventory::N_COLS * Inventory::N_ROWS) as usize
    }

    pub fn add(&mut self, item: Entity) -> bool {
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

    pub fn add_at(&mut self, item: Entity, index: usize) -> bool {
        assert!(index < Self::len());
        if self.0[index].is_some() {
            warn!("Can't add item to a non empty location");
            return false;
        }
        info!("Inventory added {item} at {index}");
        self.0[index] = Some(item);
        true
    }

    pub fn remove(&mut self, item: Entity) -> bool {
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
#[derive(Event)]
pub struct AddToInventoryEvent {
    pub item: Entity,
    pub pos: Option<usize>,
}

impl AddToInventoryEvent {
    pub fn new(item: Entity) -> Self {
        AddToInventoryEvent { item, pos: None }
    }

    pub fn new_at(item: Entity, pos: usize) -> Self {
        AddToInventoryEvent {
            item,
            pos: Some(pos),
        }
    }
}

/// Try to remove an item to the [Inventory].
#[derive(Event)]
pub struct RemoveFromInventoryEvent(pub Entity);

/// Try to add a [DroppedItem] item to the [Inventory].
#[derive(Event)]
pub struct TakeDroppedItemEvent(pub Entity);
