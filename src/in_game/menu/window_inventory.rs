use super::{
    dnd::{DndCursor, DraggedEntity, ItemEntity, ShowBorderOnDrag},
    panel_equipments::EquipmentsPanel,
    popup_info::SpawnInfoPopupObservers,
};
use crate::{
    components::{
        despawn_all,
        inventory::{AddToInventoryAtIndexCommand, Inventory, InventoryChanged},
        item::{ItemAssets, ItemInfo},
    },
    schedule::{GameRunningSet, GameState},
};
use bevy::{color::palettes::css, input::common_conditions::input_just_pressed, prelude::*};

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
        // width: Val::Px(Inventory::N_COLS as f32 * 32.),
        // height: Val::Px(Inventory::N_ROWS as f32 * 32.),
        grid_template_columns: RepeatedGridTrack::flex(Inventory::N_COLS, 1.),
        grid_template_rows: RepeatedGridTrack::flex(Inventory::N_ROWS, 1.),
        ..Default::default()
    }),
    BackgroundColor(|| BackgroundColor(css::LIGHT_GRAY.into()))
)]
pub struct InventoryPanel;

///
/// A location in the [InventoryPanel]
///
#[derive(Component)]
#[require(
    Name(|| Name::new("InventoryLocation")),
    Node(InventoryLocation::default_node),
    BackgroundColor(|| BackgroundColor(css::DARK_GRAY.into())),
    BorderColor(|| BorderColor(Srgba::NONE.into())),
)]
pub struct InventoryLocation;

impl InventoryLocation {
    fn default_node() -> Node {
        Node {
            border: UiRect::all(Val::Px(3.)),
            ..Default::default()
        }
    }

    fn node(index: usize) -> Node {
        let pos = Inventory::pos(index);
        Node {
            grid_column: GridPlacement::start(pos.col + 1),
            grid_row: GridPlacement::start(pos.row + 1),
            ..Self::default_node()
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
            }
        }
        None => {
            // spawn window as it doesn't exist
            commands.spawn(InventoryWindow).with_children(|wnd| {
                wnd.spawn(EquipmentsPanel);
                wnd.spawn(InventoryPanel);
            });
        }
    }
}

fn create_panel(trigger: Trigger<OnAdd, InventoryPanel>, mut commands: Commands) {
    let mut spawn_info_observers = SpawnInfoPopupObservers::new();
    let mut show_borders_on_drag_observers = ShowBorderOnDrag::new();
    commands.entity(trigger.entity()).with_children(|cmd| {
        for idx in 0..Inventory::len() {
            let id = cmd
                .spawn((
                    InventoryLocation,
                    Name::new(format!("InventoryLocation({idx})")),
                    InventoryLocation::node(idx),
                    InventoryIndex(idx),
                ))
                .observe(on_drop_on_location)
                .with_children(|location| {
                    let id = location
                        .spawn((
                            Name::new(format!("InventoryItem({idx})")),
                            ImageNode::default(),
                            InventoryIndex(idx),
                            ItemEntity::default(),
                        ))
                        .observe(on_drag_start_item)
                        .observe(on_drag_end)
                        .id();
                    spawn_info_observers.watch_entity(id);
                })
                .id();
            show_borders_on_drag_observers.watch_entity(id);
        }
    });

    spawn_info_observers.spawn(&mut commands);
    show_borders_on_drag_observers.spawn(&mut commands);

    commands.queue(|world: &mut World| {
        world.trigger(InventoryChanged);
    });
}

fn update_inventory(
    _trigger: Trigger<InventoryChanged>,
    mut nodes: Query<(&mut ImageNode, &mut ItemEntity, &InventoryIndex)>,
    inventory: Single<&Inventory>,
    infos: Query<&ItemInfo>,
    assets: Res<ItemAssets>,
) {
    for (mut image_node, mut item_entity, index) in &mut nodes {
        let (entity_option, item_image_node) = match inventory.at(index.0) {
            Some(item) => (
                Some(item),
                assets.image_node(
                    infos
                        .get(item)
                        .expect("Item should have ItemInfo")
                        .tile_index,
                ),
            ),
            None => (None, assets.empty_image_node()),
        };
        item_entity.0 = entity_option;
        *image_node = item_image_node;
    }
}

fn on_drag_start_item(
    trigger: Trigger<Pointer<DragStart>>,
    indexes: Query<(&InventoryIndex, &ImageNode), Without<DndCursor>>,
    inventory: Single<&Inventory>,
    cursor: Single<(&mut DraggedEntity, &mut ImageNode), With<DndCursor>>,
) {
    warn!("on_drag_start({})", trigger.entity());
    if let Ok((index, item_image)) = indexes.get(trigger.entity()) {
        if let Some(item) = inventory.at(index.0) {
            let (mut dragged_entity, mut cursor_image) = cursor.into_inner();
            **dragged_entity = Some(item);
            *cursor_image = item_image.clone();
        }
    }
}

fn on_drag_end(
    trigger: Trigger<Pointer<DragEnd>>,
    cursor: Single<(&mut DraggedEntity, &mut ImageNode), With<DndCursor>>,
) {
    warn!("on_drag_end({})", trigger.entity());
    let (mut dragged_entity, mut cursor_image) = cursor.into_inner();
    **dragged_entity = None;
    *cursor_image = ImageNode::default();
}

fn on_drop_on_location(
    trigger: Trigger<Pointer<DragDrop>>,
    mut commands: Commands,
    indexes: Query<&InventoryIndex, With<InventoryLocation>>,
    cursor: Single<&DraggedEntity, With<DndCursor>>,
    inventory: Single<&Inventory>,
) {
    warn!("on_drop_on_location({})", trigger.entity());
    let Ok(index) = indexes.get(trigger.entity()) else {
        return;
    };
    if inventory.at(index.0).is_none() {
        // There is no item at the index in the inventory
        if let Some(item) = ***cursor {
            commands.queue(AddToInventoryAtIndexCommand {
                item,
                index: index.0,
            });
        }
    }
}
