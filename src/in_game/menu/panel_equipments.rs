use super::{
    dnd::{DndCursor, DraggedEntity, ItemEntity, ShowBorderOnDrag},
    popup_info::SpawnInfoPopupObservers,
};
use crate::components::{
    equipment::{Amulet, BodyArmour, Boots, Helmet, Weapon},
    inventory::{EquipItemCommand, PlayerEquipmentChanged},
    item::{ItemAssets, ItemInfo},
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
    ImageNode,
    BackgroundColor(|| BackgroundColor(Srgba::rgb_u8(70, 70, 70).into())),
    BorderColor(|| BorderColor(Srgba::BLACK.into())),
    ItemEntity
)]
struct EquipmentBox;

#[inline]
fn default_box_node() -> Node {
    Node {
        position_type: PositionType::Absolute,
        border: UiRect::all(Val::Px(3.)),
        width: Val::Px(48.),
        height: Val::Px(48.),
        ..Default::default()
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
    let mut show_borders_on_drag_observers = ShowBorderOnDrag::new();

    commands.entity(trigger.entity()).with_children(|panel| {
        let id = panel
            .spawn(HelmetLocation::bundle(&assets))
            .observe(on_drop_equipment::<Helmet>)
            .id();
        spawn_info_observers.watch_entity(id);
        show_borders_on_drag_observers.watch_entity(id);

        let id = panel
            .spawn(BodyArmourLocation::bundle(&assets))
            .observe(on_drop_equipment::<BodyArmour>)
            .id();
        spawn_info_observers.watch_entity(id);
        show_borders_on_drag_observers.watch_entity(id);

        let id = panel
            .spawn(BootsLocation::bundle(&assets))
            .observe(on_drop_equipment::<Boots>)
            .id();
        spawn_info_observers.watch_entity(id);
        show_borders_on_drag_observers.watch_entity(id);

        let id = panel
            .spawn(AmuletLocation::bundle(&assets))
            .observe(on_drop_equipment::<Amulet>)
            .id();
        spawn_info_observers.watch_entity(id);
        show_borders_on_drag_observers.watch_entity(id);

        let id = panel
            .spawn(WeaponLocation::bundle(&assets))
            .observe(on_drop_equipment::<Weapon>)
            .id();
        spawn_info_observers.watch_entity(id);
        show_borders_on_drag_observers.watch_entity(id);
    });

    spawn_info_observers.spawn(&mut commands);
    show_borders_on_drag_observers.spawn(&mut commands);

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
    warn!(
        "on_drop_equipment::<{}>({})",
        std::any::type_name::<T>(),
        trigger.entity()
    );
    if let Some(item_entity) = ***cursor {
        warn!("on_drop_equipment() item={item_entity}");
        if equipments.get(item_entity).is_ok() {
            warn!("on_drop_equipment() item is correct");
            // The item drop is the correct one
            commands.queue(EquipItemCommand(item_entity));
        }
    }
}

fn update_equipment<F1, F2>(
    mut locations: Query<(&mut ImageNode, &mut ItemEntity), F1>,
    equipments: Query<(Entity, &ItemInfo, &Parent), F2>,
    player: Entity,
    assets: &ItemAssets,
) where
    F1: QueryFilter,
    F2: QueryFilter,
{
    let (entity_option, equipment_image_node) = equipments
        .iter()
        .filter(|(_e, _info, parent)| ***parent == player)
        .map(|(e, info, _)| (Some(e), assets.image_node(info.tile_index)))
        .map(|v| dbg!(v))
        .next() // There should be only one result if it matches
        .unwrap_or((None, assets.empty_image_node()));

    dbg!(entity_option);
    for (mut image_node, mut equipment_entity) in &mut locations {
        warn!(" *********** OK");
        *image_node = equipment_image_node.clone();
        equipment_entity.0 = entity_option;
    }
}

fn update_equipments(
    _trigger: Trigger<PlayerEquipmentChanged>,
    helmet_locations: Query<
        (&mut ImageNode, &mut ItemEntity),
        (
            With<HelmetLocation>,
            Without<BodyArmourLocation>,
            Without<BootsLocation>,
            Without<AmuletLocation>,
            Without<WeaponLocation>,
        ),
    >,
    helmets: Query<(Entity, &ItemInfo, &Parent), With<Helmet>>,
    body_armour_locations: Query<
        (&mut ImageNode, &mut ItemEntity),
        (
            With<BodyArmourLocation>,
            Without<HelmetLocation>,
            Without<BootsLocation>,
            Without<AmuletLocation>,
            Without<WeaponLocation>,
        ),
    >,
    body_armours: Query<(Entity, &ItemInfo, &Parent), With<BodyArmour>>,
    boots_locations: Query<
        (&mut ImageNode, &mut ItemEntity),
        (
            With<BootsLocation>,
            Without<HelmetLocation>,
            Without<BodyArmourLocation>,
            Without<AmuletLocation>,
            Without<WeaponLocation>,
        ),
    >,
    boots: Query<(Entity, &ItemInfo, &Parent), With<Boots>>,
    amulet_locations: Query<
        (&mut ImageNode, &mut ItemEntity),
        (
            With<AmuletLocation>,
            Without<HelmetLocation>,
            Without<BodyArmourLocation>,
            Without<BootsLocation>,
            Without<WeaponLocation>,
        ),
    >,
    amulets: Query<(Entity, &ItemInfo, &Parent), With<Amulet>>,
    weapon_locations: Query<
        (&mut ImageNode, &mut ItemEntity),
        (
            With<WeaponLocation>,
            Without<HelmetLocation>,
            Without<BodyArmourLocation>,
            Without<BootsLocation>,
            Without<AmuletLocation>,
        ),
    >,
    weapons: Query<(Entity, &ItemInfo, &Parent), With<Weapon>>,
    player: Single<Entity, With<Player>>,
    assets: Res<ItemAssets>,
) {
    warn!("update_equipments()");
    update_equipment(helmet_locations, helmets, *player, &assets);
    update_equipment(body_armour_locations, body_armours, *player, &assets);
    update_equipment(boots_locations, boots, *player, &assets);
    update_equipment(amulet_locations, amulets, *player, &assets);
    update_equipment(weapon_locations, weapons, *player, &assets);
}

pub struct InventoryPanelPlugin;

impl Plugin for InventoryPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_panel_content)
            .add_observer(update_equipments);
    }
}
