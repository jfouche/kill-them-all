use super::{
    panel_equipments::EquipmentsPanel, popup_info::ShowPopupOnMouseOver,
    popup_select_equipment::ShowEquipmentActionsOnClick,
};
use crate::{
    camera::MainCamera,
    components::{
        despawn_all,
        inventory::{Inventory, InventoryChanged, InventoryPos},
        item::{Item, ItemAssets, ItemInfo},
    },
    in_game::{GameRunningSet, GameState},
    utils::{
        dnd_ui::{Cursor, Draggable, Dragged, Hoverable},
        picking::INVENTORY_DEPTH,
    },
};
use bevy::{
    color::palettes::css,
    input::common_conditions::input_just_pressed,
    picking::{
        backend::{HitData, PointerHits},
        pointer::{PointerId, PointerLocation},
        PickSet,
    },
    prelude::*,
};

///
/// A window that shows the content of the [Inventory]
///
#[derive(Component)]
#[require(
    Name(|| Name::new("InventoryWindow")),
    Visibility(|| Visibility::Hidden),
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
    BorderColor(|| BorderColor(Srgba::NONE.into()))
)]
pub struct InventoryLocation;

#[derive(Component, Reflect)]
struct InventoryIndex(usize);

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

#[derive(Bundle)]
struct InventoryItemBundle {
    image_node: ImageNode,
    node: Node,
    on_over: ShowPopupOnMouseOver,
    draggable: Draggable,
}

impl InventoryItemBundle {
    fn new(pos: InventoryPos, info: &ItemInfo, assets: &ItemAssets) -> Self {
        InventoryItemBundle {
            image_node: assets.image_node(info.tile_index),
            node: Node {
                grid_column: GridPlacement::start(pos.col + 1),
                grid_row: GridPlacement::start(pos.row + 1),
                ..Default::default()
            },
            on_over: ShowPopupOnMouseOver {
                text: info.text.clone(),
                image: Some(assets.image_node(info.tile_index)),
            },
            draggable: Draggable,
        }
    }
}

#[derive(Bundle)]
struct InventoryEquipmentBundle {
    base: InventoryItemBundle,
    hoverable: Hoverable,
    on_click: ShowEquipmentActionsOnClick,
}

impl InventoryEquipmentBundle {
    fn new(item: Entity, pos: InventoryPos, info: &ItemInfo, assets: &ItemAssets) -> Self {
        InventoryEquipmentBundle {
            base: InventoryItemBundle::new(pos, info, assets),
            hoverable: Hoverable,
            on_click: ShowEquipmentActionsOnClick {
                text: info.text.clone(),
                image: Some(assets.image_node(info.tile_index)),
                item,
            },
        }
    }
}

#[derive(Bundle)]
struct InventoryOrbBundle {
    base: InventoryItemBundle,
}

impl InventoryOrbBundle {
    fn new(pos: InventoryPos, info: &ItemInfo, assets: &ItemAssets) -> Self {
        InventoryOrbBundle {
            base: InventoryItemBundle::new(pos, info, assets),
        }
    }
}

pub struct InventoryPanelPlugin;

impl Plugin for InventoryPanelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InventoryIndex>()
            .add_systems(Startup, spawn_window)
            .add_systems(OnExit(GameState::InGame), despawn_all::<InventoryWindow>)
            .add_systems(
                PreUpdate,
                inventory_picking_backend.in_set(PickSet::Backend),
            )
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

fn spawn_window(mut commands: Commands) {
    commands.spawn(InventoryWindow).with_children(|wnd| {
        wnd.spawn(EquipmentsPanel);
        wnd.spawn(InventoryPanel);
    });
}

fn toggle_window(window: Single<&mut Visibility, With<InventoryWindow>>) {
    let mut visibiliy = window.into_inner();
    *visibiliy = match *visibiliy {
        Visibility::Hidden => Visibility::Inherited,
        _ => Visibility::Hidden,
    }
}

fn create_panel(
    trigger: Trigger<OnAdd, InventoryPanel>,
    mut commands: Commands,
    inventory: Single<&Inventory>,
    infos: Query<&ItemInfo>,
    assets: Res<ItemAssets>,
) {
    // TODO : create observers globally
    commands.entity(trigger.entity()).with_children(|cmd| {
        for (idx, item) in inventory.iter() {
            cmd.spawn((
                InventoryLocation,
                InventoryLocation::node(idx),
                InventoryIndex(idx),
            ))
            .observe(on_over_location)
            .observe(on_out_location)
            .observe(on_drag_drop_on_location)
            .with_children(|location| {
                match item {
                    Some(item) => {
                        let info = infos.get(*item).expect("Item should have ItemInfo");
                        location.spawn((
                            assets.image_node(info.tile_index),
                            Item,
                            InventoryIndex(idx),
                        ))
                    }
                    None => {
                        // 351 is a transparent image
                        location.spawn((assets.image_node(351), InventoryIndex(idx)))
                    }
                }
                .observe(on_over_item)
                .observe(on_drag_start_item)
                .observe(on_drag_end_item);
            });
        }
    });
}

fn update_inventory(
    _trigger: Trigger<InventoryChanged>,
    mut commands: Commands,
    mut nodes: Query<(Entity, &mut ImageNode, &InventoryIndex)>,
    inventory: Single<&Inventory>,
    infos: Query<&ItemInfo>,
    assets: Res<ItemAssets>,
) {
    for (entity, mut image, index) in &mut nodes {
        match inventory.at(index.0) {
            Some(item) => {
                let info = infos.get(item).expect("Item should have ItemInfo");
                *image = assets.image_node(info.tile_index);
                commands.entity(entity).insert(Item);
            }
            None => {
                *image = assets.image_node(351);
                commands.entity(entity).remove::<Item>();
            }
        }
    }
}

fn on_over_location(
    trigger: Trigger<Pointer<Over>>,
    cursor: Query<&Children, With<Cursor>>,
    mut locations: Query<&mut BorderColor, With<InventoryLocation>>,
) {
    let Ok(mut border_color) = locations.get_mut(trigger.entity()) else {
        return;
    };
    if cursor.get_single().is_ok() {
        border_color.0 = css::YELLOW.into();
    } else {
        border_color.0 = Srgba::NONE.into();
    }
}

fn on_out_location(
    trigger: Trigger<Pointer<Out>>,
    mut locations: Query<&mut BorderColor, With<InventoryLocation>>,
) {
    let Ok(mut border_color) = locations.get_mut(trigger.entity()) else {
        return;
    };
    border_color.0 = Srgba::NONE.into();
}

fn on_over_item(trigger: Trigger<Pointer<Over>>) {
    warn!("on_over({})", trigger.entity());
}

fn on_drag_start_item(
    trigger: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    indexes: Query<&InventoryIndex, With<Item>>,
) {
    warn!("on_drag_start({})", trigger.entity());
    if indexes.get(trigger.entity()).is_ok() {
        commands.entity(trigger.entity()).insert(Dragged);
    }
}

fn on_drag_end_item(
    trigger: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    indexes: Query<&InventoryIndex>,
) {
    warn!("on_drag_end({})", trigger.entity());
    if indexes.get(trigger.entity()).is_ok() {
        commands.entity(trigger.entity()).remove::<Dragged>();
    }
}
fn on_drag_drop_on_location(
    trigger: Trigger<Pointer<DragDrop>>,
    mut commands: Commands,
    indexes: Query<&InventoryIndex>,
) {
    warn!(
        "on_drag_drop({}) on {}",
        trigger.entity(),
        trigger.event().target
    );
}

fn inventory_picking_backend(
    pointers: Query<(&PointerId, &PointerLocation)>,
    camera: Single<(Entity, &Camera), With<MainCamera>>,
    items: Query<(Entity, &GlobalTransform, &ComputedNode, &Parent), With<Item>>,
    inventory_locations: Query<(), With<InventoryLocation>>,
    mut output: EventWriter<PointerHits>,
) {
    let (camera_entity, camera) = *camera;
    let order = camera.order as f32 + 0.5;
    for (pointer_id, pointer_pos) in pointers
        .iter()
        .filter_map(|(id, loc)| loc.location().map(|l| (id, l.position)))
    {
        let mut pointer_pos = pointer_pos * camera.target_scaling_factor().unwrap_or(1.);
        if let Some(viewport) = camera.physical_viewport_rect() {
            pointer_pos -= viewport.min.as_vec2();
        }
        // warn!("pointer_pos = {pointer_pos}");
        let mut picks = Vec::new();
        for (entity, transform, node) in items
            .iter()
            .filter(|(_, _, _, p)| inventory_locations.get(p.get()).is_ok())
            .map(|(entity, transform, node, _)| (entity, transform, node))
        {
            let rect = Rect::from_center_size(transform.translation().xy(), node.size());
            if rect.contains(pointer_pos) {
                picks.push((
                    entity,
                    HitData::new(camera_entity, INVENTORY_DEPTH, None, None),
                ));
            }
        }
        if !picks.is_empty() {
            warn!("pointer: {pointer_pos} on {:?}", picks.first().unwrap().0);
            output.send(PointerHits::new(*pointer_id, picks, order));
        }
    }
}
