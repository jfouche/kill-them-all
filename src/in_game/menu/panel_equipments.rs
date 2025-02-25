use super::{
    dnd::{DndCursor, DraggedEntity, ShowBorderOnDrag},
    popup_info::SpawnInfoPopupObservers,
};
use crate::components::{
    equipment::{Amulet, BodyArmour, Boots, Helmet, Weapon},
    inventory::{EquipItemCommand, PlayerEquipmentChanged},
    item::{ItemAssets, ItemEntity, ItemInfo, ItemLocation},
    player::Player,
};
use bevy::{ecs::query::QueryFilter, prelude::*};

///
/// Equipments panel
///
#[derive(Component)]
#[require(
    Name(|| Name::new("EquipmentsPanel")),
    Node(|| Node {
        width: Val::Px(200.),
        height: Val::Px(200.),
        padding: UiRect::all(Val::Px(5.)),
        margin: UiRect::horizontal(Val::Auto),
        ..Default::default()
    }),
    BackgroundColor(|| BackgroundColor(Srgba::rgb_u8(40, 40, 40).into()))
)]
pub struct EquipmentsPanel;

///
///  A box that shows an equipment
///
#[derive(Component, Default)]
#[require(
    Name(|| Name::new("EquipmentBox")),
    ItemLocation
)]
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
    let mut spawn_info_observers = SpawnInfoPopupObservers::new();
    let mut helmet_border_obervers = ShowBorderOnDrag::<With<Helmet>>::new();
    let mut body_armour_border_observers = ShowBorderOnDrag::<With<BodyArmour>>::new();
    let mut boots_border_observers = ShowBorderOnDrag::<With<Boots>>::new();
    let mut amulet_border_observers = ShowBorderOnDrag::<With<Amulet>>::new();
    let mut weapon_border_observers = ShowBorderOnDrag::<With<Weapon>>::new();
    let mut on_drag_start_observer = Observer::new(on_drag_start_item);

    commands.entity(trigger.entity()).with_children(|panel| {
        let id = panel
            .spawn(HelmetLocation::bundle(&assets))
            .observe(on_drop_equipment::<Helmet>)
            .id();
        spawn_info_observers.watch_entity(id);
        helmet_border_obervers.watch_entity(id);
        on_drag_start_observer.watch_entity(id);

        let id = panel
            .spawn(BodyArmourLocation::bundle(&assets))
            .observe(on_drop_equipment::<BodyArmour>)
            .id();
        spawn_info_observers.watch_entity(id);
        body_armour_border_observers.watch_entity(id);
        on_drag_start_observer.watch_entity(id);

        let id = panel
            .spawn(BootsLocation::bundle(&assets))
            .observe(on_drop_equipment::<Boots>)
            .id();
        spawn_info_observers.watch_entity(id);
        boots_border_observers.watch_entity(id);
        on_drag_start_observer.watch_entity(id);

        let id = panel
            .spawn(AmuletLocation::bundle(&assets))
            .observe(on_drop_equipment::<Amulet>)
            .id();
        spawn_info_observers.watch_entity(id);
        amulet_border_observers.watch_entity(id);
        on_drag_start_observer.watch_entity(id);

        let id = panel
            .spawn(WeaponLocation::bundle(&assets))
            .observe(on_drop_equipment::<Weapon>)
            .id();
        spawn_info_observers.watch_entity(id);
        weapon_border_observers.watch_entity(id);
        on_drag_start_observer.watch_entity(id);
    });

    spawn_info_observers.spawn(&mut commands);
    helmet_border_obervers.spawn(&mut commands);
    body_armour_border_observers.spawn(&mut commands);
    boots_border_observers.spawn(&mut commands);
    amulet_border_observers.spawn(&mut commands);
    weapon_border_observers.spawn(&mut commands);
    commands.spawn(on_drag_start_observer);

    commands.queue(|world: &mut World| {
        world.trigger(PlayerEquipmentChanged);
    });
}

fn on_drag_start_item(
    trigger: Trigger<Pointer<DragStart>>,
    locations: Query<&ItemEntity, With<ItemLocation>>,
    infos: Query<&ItemInfo>,
    cursor: Single<(&mut DraggedEntity, &mut ImageNode), With<DndCursor>>,
    assets: Res<ItemAssets>,
) {
    if let Ok(ItemEntity(Some(item))) = locations.get(trigger.entity()) {
        warn!("on_drag_start_item({})", trigger.entity());
        if let Ok(info) = infos.get(*item) {
            let (mut dragged_entity, mut cursor_image) = cursor.into_inner();
            **dragged_entity = Some(*item);
            *cursor_image = assets.image_node(info.tile_index);
        }
    }
}

fn on_drop_equipment<T>(
    trigger: Trigger<Pointer<DragDrop>>,
    mut commands: Commands,
    cursor: Single<&DraggedEntity, With<DndCursor>>,
    equipments: Query<(), With<T>>,
) where
    T: Component,
{
    warn!(
        "on_drop_equipment::<{}>({})",
        std::any::type_name::<T>(),
        trigger.entity()
    );
    if let Some(item_entity) = ***cursor {
        if equipments.get(item_entity).is_ok() {
            // The item drop is the correct one
            commands.queue(EquipItemCommand(item_entity));
        }
    }
}

fn update_equipment<F1, F2>(
    mut locations: Query<&mut ItemEntity, F1>,
    equipments: Query<(Entity, &Parent), F2>,
    player: Entity,
) where
    F1: QueryFilter,
    F2: QueryFilter,
{
    let entity_option = equipments
        .iter()
        .filter(|(_e, parent)| ***parent == player)
        .map(|(e, _p)| e)
        .next() // There should be only one result if it matches
        ;

    for mut equipment_entity in &mut locations {
        equipment_entity.0 = entity_option;
    }
}

fn update_equipments(
    _trigger: Trigger<PlayerEquipmentChanged>,
    helmet_locations: Query<
        &mut ItemEntity,
        (
            With<HelmetLocation>,
            Without<BodyArmourLocation>,
            Without<BootsLocation>,
            Without<AmuletLocation>,
            Without<WeaponLocation>,
        ),
    >,
    helmets: Query<(Entity, &Parent), With<Helmet>>,
    body_armour_locations: Query<
        &mut ItemEntity,
        (
            With<BodyArmourLocation>,
            Without<HelmetLocation>,
            Without<BootsLocation>,
            Without<AmuletLocation>,
            Without<WeaponLocation>,
        ),
    >,
    body_armours: Query<(Entity, &Parent), With<BodyArmour>>,
    boots_locations: Query<
        &mut ItemEntity,
        (
            With<BootsLocation>,
            Without<HelmetLocation>,
            Without<BodyArmourLocation>,
            Without<AmuletLocation>,
            Without<WeaponLocation>,
        ),
    >,
    boots: Query<(Entity, &Parent), With<Boots>>,
    amulet_locations: Query<
        &mut ItemEntity,
        (
            With<AmuletLocation>,
            Without<HelmetLocation>,
            Without<BodyArmourLocation>,
            Without<BootsLocation>,
            Without<WeaponLocation>,
        ),
    >,
    amulets: Query<(Entity, &Parent), With<Amulet>>,
    weapon_locations: Query<
        &mut ItemEntity,
        (
            With<WeaponLocation>,
            Without<HelmetLocation>,
            Without<BodyArmourLocation>,
            Without<BootsLocation>,
            Without<AmuletLocation>,
        ),
    >,
    weapons: Query<(Entity, &Parent), With<Weapon>>,
    player: Single<Entity, With<Player>>,
) {
    warn!("update_equipments()");
    update_equipment(helmet_locations, helmets, *player);
    update_equipment(body_armour_locations, body_armours, *player);
    update_equipment(boots_locations, boots, *player);
    update_equipment(amulet_locations, amulets, *player);
    update_equipment(weapon_locations, weapons, *player);
}

pub struct InventoryPanelPlugin;

impl Plugin for InventoryPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_panel_content)
            .add_observer(update_equipments);
    }
}
