use super::popup_info::InfoPopup;
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
    });
}

///
/// A panel that shows player's equipments
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

impl EquipmentPos for Weapon {
    fn pos() -> Vec2 {
        Vec2::new(7., 74.)
    }
}

fn show_equipment<T>(
    trigger: Trigger<OnAdd, EquipmentsPanel>,
    mut commands: Commands,
    equipments: Query<(&EquipmentInfo, &Parent), With<T>>,
    players: Query<Entity, With<Player>>,
    assets: Res<EquipmentAssets>,
) where
    T: Component + EquipmentPos,
{
    let player = players.get_single().expect("Player");
    equipments
        .iter()
        .filter(|(_info, parent)| ***parent == player)
        .for_each(|(info, _parent)| {
            commands.entity(trigger.entity()).with_children(|panel| {
                let pos = T::pos();
                panel
                    .spawn((
                        InventoryBox,
                        Node {
                            left: Val::Px(pos.x),
                            top: Val::Px(pos.y),
                            ..default_inventory_box_node()
                        },
                        assets.image_node(info.tile_index),
                        info.clone(),
                        InfoPopup::new(info.text.clone()).with_image_atlas(
                            assets.image(),
                            assets.texture_atlas(info.tile_index),
                        ),
                    ));
            });
        });
}

pub fn inventory_panel_plugin(app: &mut App) {
    app.add_observer(show_equipment::<Amulet>)
        .add_observer(show_equipment::<BodyArmour>)
        .add_observer(show_equipment::<Boots>)
        .add_observer(show_equipment::<Helmet>)
        .add_observer(show_equipment::<Weapon>);
}
