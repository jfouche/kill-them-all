use super::{ShowEquipmentActionsOnMouseOver, ShowPopupOnMouseOver};
use crate::components::*;
use bevy::prelude::*;

/// A panel that shows the content of the [Inventory]
#[derive(Component)]
#[require(
    Name(|| Name::new("InventoryPanel")),
    Node(|| Node {
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::flex(N_COLS, 1.),
        grid_template_rows: RepeatedGridTrack::flex(N_ROWS, 1.),
        ..Default::default()
    }),
)]
pub struct InventoryPanel;

const N_COLS: u16 = 8;
const N_ROWS: u16 = 4;

// fn pos(index: u16) -> (u16, u16) {
//     let col = index % N_COLS;
//     let row = index / N_COLS;
//     (col, row)
// }

pub struct InventoryPanelPlugin;

impl Plugin for InventoryPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(create_inventory_panel)
            .add_observer(update_inventory);
    }
}

fn create_inventory_panel(
    trigger: Trigger<OnAdd, InventoryPanel>,
    mut commands: Commands,
    inventory: Single<Entity, With<Inventory>>,
    items: Query<(Entity, &EquipmentInfo, &Parent)>,
    assets: Res<EquipmentAssets>,
) {
    let panel = trigger.entity();
    commands.entity(panel).with_children(|cmd| {
        add_items(cmd, &items, *inventory, &assets);
    });
}

fn update_inventory(
    _trigger: Trigger<InventoryChanged>,
    mut commands: Commands,
    panels: Query<(Entity, &Children), With<InventoryPanel>>,
    inventory: Single<Entity, With<Inventory>>,
    items: Query<(Entity, &EquipmentInfo, &Parent)>,
    assets: Res<EquipmentAssets>,
) {
    for (panel, children) in &panels {
        commands.entity(panel).remove_children(&children);
        commands.entity(panel).with_children(|cmd| {
            add_items(cmd, &items, *inventory, &assets);
        });
    }
}

fn add_items(
    panel: &mut ChildBuilder,
    items: &Query<(Entity, &EquipmentInfo, &Parent)>,
    inventory: Entity,
    assets: &EquipmentAssets,
) {
    for (equipment, info, _) in items.iter().filter(|(_, _, parent)| ***parent == inventory) {
        panel.spawn((
            assets.image_node(info.tile_index),
            ShowPopupOnMouseOver {
                text: info.text.clone(),
                image: Some(assets.image_node(info.tile_index)),
            },
            ShowEquipmentActionsOnMouseOver {
                text: info.text.clone(),
                image: Some(assets.image_node(info.tile_index)),
                item: equipment,
            },
        ));
    }
}
