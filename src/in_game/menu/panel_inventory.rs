use crate::{
    components::*,
    in_game::{menu::popup_info::InfoPopup, GameRunningSet},
};
use bevy::prelude::*;

/// A panel that shows the content of the [Inventory]
#[derive(Component)]
#[require(
    Name(|| Name::new("InventoryPanel")),
    Node(|| Node {
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::flex(8, 1.),
        grid_template_rows: RepeatedGridTrack::flex(4, 1.),
        ..Default::default()
    }),
)]
pub struct InventoryPanel;

const N_COLS: u16 = 8;
const N_ROWS: u16 = 4;

fn pos(index: u16) -> (u16, u16) {
    let col = index % N_COLS;
    let row = index / N_COLS;
    (col, row)
}

pub struct InventoryPanelPlugin;

impl Plugin for InventoryPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(create_inventory_panel);
    }
}

fn create_inventory_panel(
    trigger: Trigger<OnAdd, InventoryPanel>,
    mut commands: Commands,
    inventory: Single<Entity, With<Inventory>>,
    items: Query<(&EquipmentInfo, &Parent)>,
    assets: Res<EquipmentAssets>,
) {
    warn!("create_inventory_panel");
    let panel = trigger.entity();
    let mut index = 0;
    commands.entity(panel).with_children(|cmd| {
        for (info, _) in items.iter().filter(|(_, parent)| ***parent == *inventory) {
            warn!("Add item in inventory");
            cmd.spawn((
                assets.image_node(info.tile_index),
                InfoPopup::new(info.text.clone())
                    .with_image_atlas(assets.image(), assets.texture_atlas(info.tile_index)),
            ));
        }
    });
}
