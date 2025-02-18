use super::{panel_equipments::EquipmentsPanel, popup_info::InfoPopup};
use crate::{
    components::{
        despawn_all,
        inventory::{AddToInventoryAtIndexCommand, Inventory, InventoryChanged},
        item::{ItemAssets, ItemInfo},
    },
    schedule::{GameRunningSet, GameState},
    utils::dnd_ui::Cursor,
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
    BorderColor(|| BorderColor(Srgba::NONE.into()))
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

#[derive(Component, Reflect)]
#[component(storage = "SparseSet")]
struct DraggedItem(Entity);

pub struct InventoryPanelPlugin;

impl Plugin for InventoryPanelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InventoryIndex>()
            .register_type::<DraggedItem>()
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
            .observe(on_drop_on_location)
            .with_children(|location| {
                let tile_index = match item {
                    Some(item) => {
                        infos
                            .get(*item)
                            .expect("Item should have ItemInfo")
                            .tile_index
                    }
                    None => 351, // 351 is an empty image
                };
                location
                    .spawn((assets.image_node(tile_index), InventoryIndex(idx)))
                    .observe(on_over_item)
                    .observe(on_drag_start_item)
                    .observe(on_drag_end);
            });
        }
    });
}

fn update_inventory(
    _trigger: Trigger<InventoryChanged>,
    mut nodes: Query<(&mut ImageNode, &InventoryIndex)>,
    inventory: Single<&Inventory>,
    infos: Query<&ItemInfo>,
    assets: Res<ItemAssets>,
) {
    for (mut image, index) in &mut nodes {
        let tile_index = match inventory.at(index.0) {
            Some(item) => {
                infos
                    .get(item)
                    .expect("Item should have ItemInfo")
                    .tile_index
            }
            None => 351,
        };
        *image = assets.image_node(tile_index);
    }
}

fn on_over_location(
    trigger: Trigger<Pointer<Over>>,
    cursor: Query<&ImageNode, With<Cursor>>,
    mut locations: Query<(&mut BorderColor, &InventoryIndex), With<InventoryLocation>>,
    inventory: Single<&Inventory>,
) {
    let Ok((mut border_color, index)) = locations.get_mut(trigger.entity()) else {
        return;
    };
    warn!(
        "on_over_location({}) : index: {} 3",
        trigger.entity(),
        index.0
    );
    if inventory.at(index.0).is_none() {
        if cursor.get_single().is_ok() {
            border_color.0 = css::YELLOW.into();
        }
    }
}

fn on_over_item(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    mut items: Query<&InventoryIndex, With<ImageNode>>,
    inventory: Single<&Inventory>,
    infos: Query<&ItemInfo>,
    assets: Res<ItemAssets>,
) {
    let Ok(index) = items.get_mut(trigger.entity()) else {
        return;
    };
    warn!("on_over_item({}) : index: {}", trigger.entity(), index.0);

    if let Some(item) = inventory.at(index.0) {
        if let Ok(info) = infos.get(item) {
            commands.spawn(InfoPopup {
                image: Some(assets.image_node(info.tile_index)),
                text: info.text.clone(),
                source: trigger.entity(),
                pos: trigger.event().pointer_location.position,
            });
        }
    }
}

fn on_out_location(
    trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    mut locations: Query<&mut BorderColor, With<InventoryLocation>>,
    popups: Query<Entity, With<InfoPopup>>,
) {
    warn!("on_out_location({})", trigger.entity());
    if let Ok(mut border_color) = locations.get_mut(trigger.entity()) {
        border_color.0 = Srgba::NONE.into();
    }
    for entity in &popups {
        commands.entity(entity).despawn_recursive();
    }
}

// fn on_over_item(trigger: Trigger<Pointer<Over>>) {
//     warn!("on_over({})", trigger.entity());
// }

fn on_drag_start_item(
    trigger: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    indexes: Query<(&InventoryIndex, &ImageNode)>,
    inventory: Single<&Inventory>,
    cursor: Single<Entity, With<Cursor>>,
) {
    warn!("on_drag_start({})", trigger.entity());
    if let Ok((index, image_node)) = indexes.get(trigger.entity()) {
        if let Some(item) = inventory.at(index.0) {
            commands.entity(*cursor).insert(image_node.clone());
            commands.entity(trigger.entity()).insert(DraggedItem(item));
        }
    }
}

fn on_drag_end(
    trigger: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    cursor: Single<Entity, With<Cursor>>,
) {
    warn!("on_drag_end({})", trigger.entity());
    commands.entity(*cursor).remove::<ImageNode>();
    commands.entity(trigger.entity()).remove::<DraggedItem>();
    commands.trigger(InventoryChanged);
}

fn on_drop_on_location(
    trigger: Trigger<Pointer<DragDrop>>,
    mut commands: Commands,
    indexes: Query<&InventoryIndex, With<InventoryLocation>>,
    dragged_items: Query<&DraggedItem>,
    inventory: Single<&Inventory>,
) {
    warn!(
        "on_drop_on_location({} on {})",
        trigger.event().target,
        trigger.entity(),
    );
    let Ok(index) = indexes.get(trigger.entity()) else {
        return;
    };
    warn!("on_drop_on_location() 1 - {}", index.0);
    if inventory.at(index.0).is_none() {
        warn!("on_drop_on_location() 2");
        if let Ok(item) = dragged_items.get_single() {
            warn!("on_drop_on_location() 3 - {}", item.0);
            commands.queue(AddToInventoryAtIndexCommand {
                item: item.0,
                index: index.0,
            });
        }
    }
}
