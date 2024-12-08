use crate::components::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

///
/// Inventory panel
///
#[derive(Component)]
#[component(on_add = create_inventory_panel)]
#[require(
    Name(|| Name::new("InventoryPanel")),
    Node(|| Node {
        padding: UiRect::all(Val::Px(5.)),
        ..Default::default()
    })
)]
pub struct InventoryPanel;

fn create_inventory_panel(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
    world.commands().entity(entity).with_children(|panel| {
        panel.spawn(EquipmentsPanel);
        panel.spawn(AffixesPanel);
    });
}

///
/// A panel that show player's equipments
///
#[derive(Component)]
#[require(
    Name(|| Name::new("EquipmentsPanel")),
    Node(|| Node {
        width: Val::Px(200.),
        height: Val::Px(200.),
        ..Default::default()
    }),
    BackgroundColor(|| BackgroundColor(Srgba::rgb_u8(40, 40, 40).into()))
)]
struct EquipmentsPanel;

///
///  A panel that shows an equipment
///
#[derive(Component)]
#[require(
    Name(|| Name::new("InventoryBox")),
    ImageNode,
    BackgroundColor(|| BackgroundColor(Srgba::rgb_u8(70, 70, 70).into())),
    Interaction
)]
struct InventoryBox;

#[inline]
fn default_inventory_box_node() -> Node {
    Node {
        position_type: PositionType::Absolute,
        width: Val::Px(48.),
        height: Val::Px(48.),
        ..Default::default()
    }
}

///
///  A panel that shows selected equipment affixes
///
#[derive(Component)]
#[require(
    Name(|| Name::new("AffixesPanel")),
    Text,
    TextFont(|| TextFont::from_font_size(16.)),
    Node(|| Node {
        margin: UiRect::all(Val::Px(5.)),
        width: Val::Px(180.),
        ..Default::default()
    })
)]
struct AffixesPanel;

///
/// Trait to get position of a specific equipment on the [EquipmentsPanel]
///
/// weapon : Vec2::new(7., 74.)
/// ?      : Vec2::new(142., 74.)
/// ?      : Vec2::new(7., 142.)
trait EquipmentPos {
    fn pos() -> Vec2;
}

impl EquipmentPos for Amulet {
    fn pos() -> Vec2 {
        Vec2::new(142., 7.)
    }
}

impl EquipmentPos for BodyArmour {
    fn pos() -> Vec2 {
        Vec2::new(74., 74.)
    }
}

impl EquipmentPos for Boots {
    fn pos() -> Vec2 {
        Vec2::new(74., 142.)
    }
}

impl EquipmentPos for Helmet {
    fn pos() -> Vec2 {
        Vec2::new(74., 7.)
    }
}

fn show_equipment<T>(
    trigger: Trigger<OnAdd, EquipmentsPanel>,
    mut commands: Commands,
    equipments: Query<(&TileIndex, &AffixesLabels, &Parent), With<T>>,
    players: Query<Entity, With<Player>>,
    assets: Res<EquipmentAssets>,
) where
    T: Component + EquipmentPos,
{
    let player = players.get_single().expect("Player");
    equipments
        .iter()
        .filter(|(_tile_index, _labels, parent)| ***parent == player)
        .for_each(|(tile_index, labels, _parent)| {
            commands.entity(trigger.entity()).with_children(|panel| {
                let pos = T::pos();
                panel
                    .spawn((
                        InventoryBox,
                        assets.image_node(**tile_index),
                        Node {
                            left: Val::Px(pos.x),
                            top: Val::Px(pos.y),
                            ..default_inventory_box_node()
                        },
                        labels.clone(),
                    ))
                    .observe(show_affixes)
                    .observe(hide_affixes);
            });
        });
}

fn hide_affixes(_trigger: Trigger<Pointer<Out>>, mut panels: Query<&mut Text, With<AffixesPanel>>) {
    if let Ok(mut text) = panels.get_single_mut() {
        *text = "".into();
    }
}

fn show_affixes(
    trigger: Trigger<Pointer<Over>>,
    equipments: Query<&AffixesLabels>,
    mut panels: Query<&mut Text, With<AffixesPanel>>,
) {
    if let Ok(mut text) = panels.get_single_mut() {
        if let Ok(labels) = equipments.get(trigger.entity()) {
            *text = labels.into();
        }
    };
}

pub fn inventory_panel_plugin(app: &mut App) {
    app.add_observer(show_equipment::<Amulet>)
        .add_observer(show_equipment::<BodyArmour>)
        .add_observer(show_equipment::<Boots>)
        .add_observer(show_equipment::<Helmet>);
}
