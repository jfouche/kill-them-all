use super::{panel_equipments::EquipmentsPanel, panel_skills::skills_panel};
use crate::{
    components::{
        despawn_all,
        inventory::{
            AddToInventoryEvent, Inventory, InventoryChanged, PlayerEquipmentChanged,
            ToggleInventory,
        },
        item::{ItemEntity, ItemLocation, ItemLocationAcceptAll},
        orb::{ActivateOrbEvent, Orb},
    },
    in_game::dnd::{DndCursor, DraggedEntity},
    schedule::{GameRunningSet, GameState},
};
use bevy::{ecs::query::QuerySingleError, prelude::*};

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
            .add_observer(toggle_window)
            .add_observer(on_drop_on_location);
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
    match windows.single_mut() {
        Ok(mut visiblity) => {
            *visiblity = match *visiblity {
                Visibility::Hidden => Visibility::Inherited,
                _ => Visibility::Hidden,
            };
        }
        Err(QuerySingleError::NoEntities(_)) => {
            // spawn window as it doesn't exist
            commands.spawn((
                InventoryWindow,
                children![EquipmentsPanel, skills_panel(), InventoryPanel],
            ));
        }
        _ => unreachable!(),
    }
    commands.trigger(PlayerEquipmentChanged);
}

fn create_panel(trigger: Trigger<OnAdd, InventoryPanel>, mut commands: Commands) {
    let panel = trigger.target();
    for idx in 0..Inventory::len() {
        commands.spawn((
            InventoryLocation,
            ItemLocationAcceptAll,
            Name::new(format!("InventoryLocation({idx})")),
            InventoryLocation::node(idx),
            InventoryIndex(idx),
            ChildOf(panel),
        ));
    }
    commands.trigger(InventoryChanged);
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
