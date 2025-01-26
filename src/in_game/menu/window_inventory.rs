use super::{
    panel_equipments::EquipmentsPanel, popup_info::ShowPopupOnMouseOver,
    popup_select_equipment::ShowEquipmentActionsOnClick,
};
use crate::{
    components::{
        despawn_all,
        equipment::Equipment,
        inventory::{Inventory, InventoryChanged, InventoryPos},
        item::{ItemAssets, ItemInfo},
        orb::Orb,
    },
    in_game::{GameRunningSet, GameState},
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

/// A window that shows the content of the [Inventory]
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

/// A panel that shows the content of the [Inventory]
#[derive(Component)]
#[require(
    Name(|| Name::new("InventoryPanel")),
    Node(|| Node {
        display: Display::Grid,
        width: Val::Px(Inventory::N_COLS as f32 * 32.),
        height: Val::Px(Inventory::N_ROWS as f32 * 32.),
        grid_template_columns: RepeatedGridTrack::flex(Inventory::N_COLS, 1.),
        grid_template_rows: RepeatedGridTrack::flex(Inventory::N_ROWS, 1.),
        ..Default::default()
    }),
)]
pub struct InventoryPanel;

#[derive(Bundle)]
struct InventoryItemBundle {
    image_node: ImageNode,
    node: Node,
    on_over: ShowPopupOnMouseOver,
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
        }
    }
}

#[derive(Bundle)]
struct InventoryEquipmentBundle {
    base: InventoryItemBundle,
    on_click: ShowEquipmentActionsOnClick,
}

impl InventoryEquipmentBundle {
    fn new(item: Entity, pos: InventoryPos, info: &ItemInfo, assets: &ItemAssets) -> Self {
        InventoryEquipmentBundle {
            base: InventoryItemBundle::new(pos, info, assets),
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
        app.add_systems(OnExit(GameState::InGame), despawn_all::<InventoryWindow>)
            .add_systems(
                Update,
                spawn_or_despawn_window
                    .run_if(input_just_pressed(KeyCode::KeyI))
                    .in_set(GameRunningSet::UserInput),
            )
            .add_observer(create_panel)
            .add_observer(update_inventory);
    }
}

fn spawn_or_despawn_window(mut commands: Commands, windows: Query<Entity, With<InventoryWindow>>) {
    if let Ok(entity) = windows.get_single() {
        commands.entity(entity).despawn_recursive();
    } else {
        commands.spawn(InventoryWindow).with_children(|wnd| {
            wnd.spawn(EquipmentsPanel);
            wnd.spawn(InventoryPanel);
        });
    }
}

fn create_panel(
    trigger: Trigger<OnAdd, InventoryPanel>,
    mut commands: Commands,
    inventory: Single<&Inventory>,
    equipments: Query<&ItemInfo, With<Equipment>>,
    orbs: Query<&ItemInfo, With<Orb>>,
    assets: Res<ItemAssets>,
) {
    commands.entity(trigger.entity()).with_children(|cmd| {
        for (item, pos) in inventory.iter() {
            if let Ok(info) = equipments.get(item) {
                cmd.spawn(InventoryEquipmentBundle::new(item, pos, info, &assets));
            } else if let Ok(info) = orbs.get(item) {
                cmd.spawn(InventoryOrbBundle::new(pos, info, &assets));
            }
        }
    });
}

fn update_inventory(
    _trigger: Trigger<InventoryChanged>,
    mut commands: Commands,
    panels: Query<Entity, With<InventoryPanel>>,
    inventory: Single<&Inventory>,
    equipments: Query<&ItemInfo, With<Equipment>>,
    orbs: Query<&ItemInfo, With<Orb>>,
    assets: Res<ItemAssets>,
) {
    for panel in &panels {
        commands.entity(panel).despawn_descendants();
        commands.entity(panel).with_children(|cmd| {
            for (item, pos) in inventory.iter() {
                if let Ok(info) = equipments.get(item) {
                    cmd.spawn(InventoryEquipmentBundle::new(item, pos, info, &assets));
                } else if let Ok(info) = orbs.get(item) {
                    cmd.spawn(InventoryOrbBundle::new(pos, info, &assets));
                }
            }
        });
    }
}
