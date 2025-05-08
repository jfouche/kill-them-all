use super::{
    dnd::{DndCursor, DraggedEntity},
    item_location::{ItemLocationDragObservers, ShowBorderOnDrag},
    panel_equipments::EquipmentsPanel,
    panel_skills::SkillsPanel,
};
use crate::{
    components::{
        despawn_all,
        inventory::{
            AddToInventoryEvent, Inventory, InventoryChanged, PlayerEquipmentChanged,
            ToggleInventory,
        },
        item::{ItemEntity, ItemLocation},
        orb::{ActivateOrbEvent, Orb},
    },
    schedule::{GameRunningSet, GameState},
    utils::observers::VecObserversExt,
};
use bevy::prelude::*;

///
/// A window that shows the content of the [Inventory]
///
#[derive(Component)]
#[require(
    Name::new("InventoryWindow"),
    Node {
        position_type: PositionType::Absolute,
        flex_direction: FlexDirection::Column,
        right: Val::Px(0.),
        bottom: Val::Px(50.),
        border: UiRect::all(Val::Px(1.)),
        ..Default::default()
    },
    BorderColor(Color::BLACK),
    BackgroundColor(Color::srgb(0.5, 0.5, 0.5))
)]
pub struct InventoryWindow;

///
/// A panel that shows the content of the [Inventory]
///
#[derive(Component)]
#[require(
    Name::new("InventoryPanel"),
    Node {
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::flex(Inventory::N_COLS, 1.),
        grid_template_rows: RepeatedGridTrack::flex(Inventory::N_ROWS, 1.),
        ..Default::default()
    },
    BackgroundColor(Srgba::rgb(0.16, 0.16, 0.16).into())
)]
pub struct InventoryPanel;

///
/// A location in the [InventoryPanel]
///
#[derive(Component)]
#[require(ItemLocation)]
struct InventoryLocation;

impl InventoryLocation {
    fn node(index: usize) -> Node {
        let pos = Inventory::pos(index);
        Node {
            grid_column: GridPlacement::start(pos.col + 1),
            grid_row: GridPlacement::start(pos.row + 1),
            ..ItemLocation::default_node()
        }
    }
}

#[derive(Component, Reflect)]
struct InventoryIndex(usize);

pub struct InventoryPanelPlugin;

impl Plugin for InventoryPanelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InventoryIndex>()
            .add_systems(OnExit(GameState::InGame), despawn_all::<InventoryWindow>)
            .add_systems(
                Update,
                trigger_toggle_window.in_set(GameRunningSet::UserInput),
            )
            .add_observer(create_panel)
            .add_observer(update_inventory)
            .add_observer(toggle_window);
    }
}

fn trigger_toggle_window(
    mut commands: Commands,
    windows: Query<&Visibility, With<InventoryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyI)
        || (keys.just_pressed(KeyCode::Space) && windows.iter().next() != Some(&Visibility::Hidden))
    {
        commands.trigger(ToggleInventory);
    }
}

fn toggle_window(
    _: Trigger<ToggleInventory>,
    mut commands: Commands,
    mut windows: Query<&mut Visibility, With<InventoryWindow>>,
) {
    match windows.iter_mut().next() {
        Some(mut visiblity) => {
            *visiblity = match *visiblity {
                Visibility::Hidden => Visibility::Inherited,
                _ => Visibility::Hidden,
            };
            commands.trigger(PlayerEquipmentChanged);
        }
        None => {
            // spawn window as it doesn't exist
            commands.spawn((
                InventoryWindow,
                children![EquipmentsPanel, SkillsPanel, InventoryPanel],
            ));
        }
    }
}

fn create_panel(trigger: Trigger<OnAdd, InventoryPanel>, mut commands: Commands) {
    let mut observers = vec![Observer::new(on_drop_on_location)]
        .with_observers(ItemLocationDragObservers::observers())
        .with_observers(<ShowBorderOnDrag>::observers());

    commands.entity(trigger.target()).with_children(|cmd| {
        for idx in 0..Inventory::len() {
            let entity = cmd
                .spawn((
                    InventoryLocation,
                    Name::new(format!("InventoryLocation({idx})")),
                    InventoryLocation::node(idx),
                    InventoryIndex(idx),
                ))
                .id();
            observers.watch_entity(entity);
        }
    });

    commands.spawn_batch(observers);

    commands.queue(|world: &mut World| {
        world.trigger(InventoryChanged);
    });
}

fn update_inventory(
    _trigger: Trigger<InventoryChanged>,
    mut nodes: Query<(&mut ItemEntity, &InventoryIndex)>,
    inventory: Single<&Inventory>,
) {
    for (mut item_entity, index) in &mut nodes {
        item_entity.0 = inventory.at(index.0);
    }
}

fn on_drop_on_location(
    trigger: Trigger<Pointer<DragDrop>>,
    mut commands: Commands,
    indexes: Query<&InventoryIndex, With<InventoryLocation>>,
    cursor: Single<&DraggedEntity, With<DndCursor>>,
    orbs: Query<(), With<Orb>>,
    inventory: Single<&Inventory>,
) {
    let location_item = trigger.target();
    let Some(drop_item) = ***cursor else {
        warn!("on_drop_on_location({location_item}) without item on cursor",);
        return;
    };
    let Ok(index) = indexes.get(location_item) else {
        warn!("on_drop_on_location({location_item}) without InventoryLocation",);
        return;
    };
    match inventory.at(index.0) {
        None => {
            // There is no item at the index in the inventory
            info!("on_drop_on_location({location_item}) drop item {drop_item}");
            commands.trigger(AddToInventoryEvent::new_at(drop_item, index.0));
            commands.trigger(PlayerEquipmentChanged);
        }
        Some(target_item) => {
            info!("on_drop_on_location({location_item}) drop item {drop_item} on {target_item}");
            if orbs.get(drop_item).is_ok() {
                commands.trigger(ActivateOrbEvent {
                    orb: drop_item,
                    item: target_item,
                });
            }
        }
    }
}
