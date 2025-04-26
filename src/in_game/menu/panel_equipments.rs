use super::{
    dnd::{DndCursor, DraggedEntity},
    item_location::{ItemLocationDragObservers, ShowBorderOnDrag},
    popup_info::SpawnInfoPopupObservers,
};
use crate::{
    components::{
        equipment::{Amulet, BodyArmour, Boots, Helmet, Weapon},
        inventory::PlayerEquipmentChanged,
        item::{EquipEquipmentCommand, ItemAssets, ItemEntity, ItemLocation},
        player::Player,
    },
    utils::observers::VecObserversExt,
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

///
///  A box that shows an equipment
///
#[derive(Component, Default)]
#[require(Name::new("EquipmentBox"), ItemLocation)]
struct EquipmentBox;

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
#[require(EquipmentBox)]
struct HelmetLocation;

impl HelmetLocation {
    fn bundle(assets: &ItemAssets) -> impl Bundle {
        (HelmetLocation, box_node(74., 7.), assets.empty_image_node())
    }
}

#[derive(Component)]
#[require(EquipmentBox)]
struct BodyArmourLocation;

impl BodyArmourLocation {
    fn bundle(assets: &ItemAssets) -> impl Bundle {
        (
            BodyArmourLocation,
            box_node(74., 74.),
            assets.empty_image_node(),
        )
    }
}

#[derive(Component)]
#[require(EquipmentBox)]
struct BootsLocation;

impl BootsLocation {
    fn bundle(assets: &ItemAssets) -> impl Bundle {
        (
            BootsLocation,
            box_node(74., 142.),
            assets.empty_image_node(),
        )
    }
}

#[derive(Component)]
#[require(EquipmentBox)]
struct AmuletLocation;

impl AmuletLocation {
    fn bundle(assets: &ItemAssets) -> impl Bundle {
        (
            AmuletLocation,
            box_node(142., 7.),
            assets.empty_image_node(),
        )
    }
}

#[derive(Component)]
#[require(EquipmentBox)]
struct WeaponLocation;

impl WeaponLocation {
    fn bundle(assets: &ItemAssets) -> impl Bundle {
        (WeaponLocation, box_node(7., 74.), assets.empty_image_node())
    }
}

fn spawn_panel_content(
    trigger: Trigger<OnAdd, EquipmentsPanel>,
    mut commands: Commands,
    assets: Res<ItemAssets>,
) {
    let mut observers = Vec::new()
        .with_observers(SpawnInfoPopupObservers::observers())
        .with_observers(ItemLocationDragObservers::observers());

    let mut helmet_border_observers = ShowBorderOnDrag::<With<Helmet>>::default();
    let mut body_armour_border_observers = ShowBorderOnDrag::<With<BodyArmour>>::default();
    let mut boots_border_observers = ShowBorderOnDrag::<With<Boots>>::default();
    let mut amulet_border_observers = ShowBorderOnDrag::<With<Amulet>>::default();
    let mut weapon_border_observers = ShowBorderOnDrag::<With<Weapon>>::default();

    commands.entity(trigger.target()).with_children(|panel| {
        let entity = panel
            .spawn(HelmetLocation::bundle(&assets))
            .observe(on_drop_equipment::<Helmet>)
            .id();
        observers.watch_entity(entity);
        helmet_border_observers.watch_entity(entity);

        let entity = panel
            .spawn(BodyArmourLocation::bundle(&assets))
            .observe(on_drop_equipment::<BodyArmour>)
            .id();
        observers.watch_entity(entity);
        body_armour_border_observers.watch_entity(entity);

        let entity = panel
            .spawn(BootsLocation::bundle(&assets))
            .observe(on_drop_equipment::<Boots>)
            .id();
        observers.watch_entity(entity);
        boots_border_observers.watch_entity(entity);

        let entity = panel
            .spawn(AmuletLocation::bundle(&assets))
            .observe(on_drop_equipment::<Amulet>)
            .id();
        observers.watch_entity(entity);
        amulet_border_observers.watch_entity(entity);

        let entity = panel
            .spawn(WeaponLocation::bundle(&assets))
            .observe(on_drop_equipment::<Weapon>)
            .id();
        observers.watch_entity(entity);
        weapon_border_observers.watch_entity(entity);
    });

    commands.spawn_batch(observers);
    commands.spawn_batch(helmet_border_observers.0);
    commands.spawn_batch(body_armour_border_observers.0);
    commands.spawn_batch(boots_border_observers.0);
    commands.spawn_batch(amulet_border_observers.0);
    commands.spawn_batch(weapon_border_observers.0);

    commands.queue(|world: &mut World| {
        world.trigger(PlayerEquipmentChanged);
    });
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
            commands.queue(EquipEquipmentCommand(item_entity));
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
