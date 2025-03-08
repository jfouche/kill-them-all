use super::{
    dnd::{DndCursor, DraggedEntity},
    item_location::{ItemLocationDragObservers, ShowBorderOnDrag},
    panel_equipments::EquipmentsPanel,
    panel_skills::SkillsPanel,
    popup_info::SpawnInfoPopupObservers,
};
use crate::{
    components::{
        despawn_all,
        inventory::{
            AddToInventoryAtIndexCommand, Inventory, InventoryChanged, PlayerEquipmentChanged,
        },
        item::{ItemEntity, ItemLocation},
    },
    schedule::{GameRunningSet, GameState}, utils::observers::VecObserversExt,
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

///
/// A window that shows the content of the [Inventory]
///
#[derive(Component)]
#[require(
    Name(|| Name::new("InventoryWindow")),
    Node(|| Node {
        position_type: PositionType::Absolute,
        flex_direction: FlexDirection::Column,
        right: Val::Px(0.),
        bottom: Val::Px(0.),
        border: UiRect::all(Val::Px(1.)),
        ..Default::default()
    }),
    BorderColor(|| BorderColor(Color::BLACK)),
    BackgroundColor(|| BackgroundColor(Color::srgb(0.5, 0.5, 0.5)))
)]
pub struct InventoryWindow;

///
/// A panel that shows the content of the [Inventory]
///
#[derive(Component)]
#[require(
    Name(|| Name::new("InventoryPanel")),
    Node(|| Node {
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::flex(Inventory::N_COLS, 1.),
        grid_template_rows: RepeatedGridTrack::flex(Inventory::N_ROWS, 1.),
        ..Default::default()
    }),
    BackgroundColor(|| BackgroundColor(Srgba::rgb(0.16, 0.16, 0.16).into())) 
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
                toggle_window
                    .run_if(input_just_pressed(KeyCode::KeyI))
                    .in_set(GameRunningSet::UserInput),
            )
            .add_observer(create_panel)
            .add_observer(update_inventory);
    }
}

fn toggle_window(
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
            commands.spawn(InventoryWindow).with_children(|wnd| {
                wnd.spawn(EquipmentsPanel);
                wnd.spawn(SkillsPanel);
                wnd.spawn(InventoryPanel);
            });
        }
    }
}

fn create_panel(trigger: Trigger<OnAdd, InventoryPanel>, mut commands: Commands) {
    let mut observers = vec![Observer::new(on_drop_on_location)]
        .with_observers(SpawnInfoPopupObservers::observers())
        .with_observers(ItemLocationDragObservers::observers())
        .with_observers(<ShowBorderOnDrag>::observers());

    commands.entity(trigger.entity()).with_children(|cmd| {
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

// fn on_drag_start_item(
//     trigger: Trigger<Pointer<DragStart>>,
//     indexes: Query<&InventoryIndex, Without<DndCursor>>,
//     inventory: Single<&Inventory>,
//     infos: Query<&ItemInfo>,
//     cursor: Single<(&mut DraggedEntity, &mut ImageNode), With<DndCursor>>,
//     assets: Res<ItemAssets>,
// ) {
//     if let Ok(index) = indexes.get(trigger.entity()) {
//         if let Some(item) = inventory.at(index.0) {
//             warn!("on_drag_start_item({})", trigger.entity());
//             if let Ok(info) = infos.get(item) {
//                 let (mut dragged_entity, mut cursor_image) = cursor.into_inner();
//                 **dragged_entity = Some(item);
//                 *cursor_image = assets.image_node(info.tile_index);
//             }
//         }
//     }
// }

// fn on_drag_end(
//     trigger: Trigger<Pointer<DragEnd>>,
//     cursor: Single<(&mut DraggedEntity, &mut ImageNode), With<DndCursor>>,
// ) {
//     warn!("on_drag_end({})", trigger.entity());
//     let (mut dragged_entity, mut cursor_image) = cursor.into_inner();
//     **dragged_entity = None;
//     *cursor_image = ImageNode::default();
// }

fn on_drop_on_location(
    trigger: Trigger<Pointer<DragDrop>>,
    mut commands: Commands,
    indexes: Query<&InventoryIndex, With<InventoryLocation>>,
    cursor: Single<&DraggedEntity, With<DndCursor>>,
    inventory: Single<&Inventory>,
) {
    info!("on_drop_on_location({})", trigger.entity());
    if let Ok(index) = indexes.get(trigger.entity()) {
        if inventory.at(index.0).is_none() {
            // There is no item at the index in the inventory
            if let Some(item) = ***cursor {
                commands.queue(AddToInventoryAtIndexCommand {
                    item,
                    index: index.0,
                });
                commands.queue(|world: &mut World| {
                    world.trigger(PlayerEquipmentChanged);
                });
            }
        }
    }
}
