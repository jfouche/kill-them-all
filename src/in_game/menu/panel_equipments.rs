use super::dnd::{DndCursor, DraggedEntity};
use crate::components::{
    equipment::{Amulet, BodyArmour, Boots, Helmet, Weapon},
    inventory::PlayerEquipmentChanged,
    item::{EquipEquipmentEvent, ItemAssets, ItemEntity, ItemLocation, ItemLocationAccept},
    player::Player,
};
use bevy::prelude::*;

///
/// Equipments panel
///
#[derive(Component)]
#[require(
    Name::new("EquipmentsPanel"),
    Node {
        width: Val::Px(200.),
        height: Val::Px(200.),
        padding: UiRect::all(Val::Px(5.)),
        margin: UiRect::horizontal(Val::Auto),
        ..Default::default()
    },
    BackgroundColor(Srgba::rgb_u8(40, 40, 40).into())
)]
pub struct EquipmentsPanel;

#[inline]
fn default_box_node() -> Node {
    Node {
        position_type: PositionType::Absolute,
        width: Val::Px(48.),
        height: Val::Px(48.),
        ..ItemLocation::default_node()
    }
}

#[inline]
fn box_node(x: f32, y: f32) -> Node {
    Node {
        left: Val::Px(x),
        top: Val::Px(y),
        ..default_box_node()
    }
}

#[derive(Component)]
struct HelmetLocation;

fn helmet_location(assets: &ItemAssets) -> impl Bundle {
    (
        HelmetLocation,
        ItemLocation,
        ItemLocationAccept::<Helmet>::new(),
        box_node(74., 7.),
        assets.empty_image_node(),
    )
}

#[derive(Component)]
struct BodyArmourLocation;

fn body_armour_location(assets: &ItemAssets) -> impl Bundle {
    (
        BodyArmourLocation,
        ItemLocation,
        ItemLocationAccept::<BodyArmour>::new(),
        box_node(74., 74.),
        assets.empty_image_node(),
    )
}

#[derive(Component)]
struct BootsLocation;

fn boots_location(assets: &ItemAssets) -> impl Bundle {
    (
        BootsLocation,
        ItemLocation,
        ItemLocationAccept::<Boots>::new(),
        box_node(74., 142.),
        assets.empty_image_node(),
    )
}

#[derive(Component)]
struct AmuletLocation;

fn amulet_location(assets: &ItemAssets) -> impl Bundle {
    (
        AmuletLocation,
        ItemLocation,
        ItemLocationAccept::<Amulet>::new(),
        box_node(142., 7.),
        assets.empty_image_node(),
    )
}

#[derive(Component)]
struct WeaponLocation;

fn weapon_location(assets: &ItemAssets) -> impl Bundle {
    (
        WeaponLocation,
        ItemLocation,
        ItemLocationAccept::<Weapon>::new(),
        box_node(7., 74.),
        assets.empty_image_node(),
    )
}

fn spawn_panel_content(
    trigger: Trigger<OnAdd, EquipmentsPanel>,
    mut commands: Commands,
    assets: Res<ItemAssets>,
) {
    let panel = trigger.target();
    commands
        .spawn((helmet_location(&assets), ChildOf(panel)))
        .observe(on_drop_equipment::<Helmet>);

    commands
        .spawn((body_armour_location(&assets), ChildOf(panel)))
        .observe(on_drop_equipment::<BodyArmour>);

    commands
        .spawn((boots_location(&assets), ChildOf(panel)))
        .observe(on_drop_equipment::<Boots>);

    commands
        .spawn((amulet_location(&assets), ChildOf(panel)))
        .observe(on_drop_equipment::<Amulet>);

    commands
        .spawn((weapon_location(&assets), ChildOf(panel)))
        .observe(on_drop_equipment::<Weapon>);

    commands.trigger(PlayerEquipmentChanged);
}

fn on_drop_equipment<T>(
    trigger: Trigger<Pointer<DragDrop>>,
    mut commands: Commands,
    cursor: Single<&DraggedEntity, With<DndCursor>>,
    equipments: Query<(), With<T>>,
) where
    T: Component,
{
    info!(
        "on_drop_equipment::<{}>({})",
        std::any::type_name::<T>(),
        trigger.target()
    );
    if let Some(item_entity) = ***cursor {
        if equipments.get(item_entity).is_ok() {
            // The item drop is the correct one
            commands.trigger(EquipEquipmentEvent(item_entity));
        }
    }
}

fn update_equipment<EL, E>(
    _trigger: Trigger<PlayerEquipmentChanged>,
    mut locations: Query<&mut ItemEntity, With<EL>>,
    equipments: Query<(Entity, &ChildOf), With<E>>,
    player: Single<Entity, With<Player>>,
) where
    EL: Component,
    E: Component,
{
    let entity_option = equipments
        .iter()
        .filter(|(_e, child_of)| child_of.parent() == *player)
        .map(|(e, _p)| e)
        .next() // There should be only one result if it matches
        ;

    for mut equipment_entity in &mut locations {
        equipment_entity.0 = entity_option;
    }
}

pub struct EquipmentPanelPlugin;

impl Plugin for EquipmentPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_panel_content)
            .add_observer(update_equipment::<HelmetLocation, Helmet>)
            .add_observer(update_equipment::<BodyArmourLocation, BodyArmour>)
            .add_observer(update_equipment::<BootsLocation, Boots>)
            .add_observer(update_equipment::<AmuletLocation, Amulet>)
            .add_observer(update_equipment::<WeaponLocation, Weapon>);
    }
}
