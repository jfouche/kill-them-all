use crate::{
    components::{
        equipment::{Amulet, BodyArmour, Boots, Helmet, Weapon},
        inventory::{EquipItemCommand, PlayerEquipmentChanged},
        item::{ItemAssets, ItemInfo},
        player::Player,
    },
    in_game::menu::popup_info::InfoPopup,
    utils::dnd_ui::{DndCursor, DraggedEntity},
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
    EquipmentEntity,
    Interaction
)]
struct EquipmentBox;

#[inline]
fn default_box_node() -> Node {
    Node {
        position_type: PositionType::Absolute,
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

#[derive(Component, Default, Reflect)]
struct EquipmentEntity(Option<Entity>);

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
    let mut on_over_observer = Observer::new(on_over_equipment);
    let mut on_out_observer = Observer::new(on_out_equipment);

    commands.entity(trigger.entity()).with_children(|panel| {
        let id = panel
            .spawn(HelmetLocation::bundle(&assets))
            .observe(on_drop_equipment::<Helmet>)
            .id();
        on_over_observer.watch_entity(id);
        on_out_observer.watch_entity(id);

        let id = panel
            .spawn(BodyArmourLocation::bundle(&assets))
            .observe(on_drop_equipment::<BodyArmour>)
            .id();
        on_over_observer.watch_entity(id);
        on_out_observer.watch_entity(id);

        let id = panel
            .spawn(BootsLocation::bundle(&assets))
            .observe(on_drop_equipment::<Boots>)
            .id();
        on_over_observer.watch_entity(id);
        on_out_observer.watch_entity(id);

        let id = panel
            .spawn(AmuletLocation::bundle(&assets))
            .observe(on_drop_equipment::<Amulet>)
            .id();
        on_over_observer.watch_entity(id);
        on_out_observer.watch_entity(id);

        let id = panel
            .spawn(WeaponLocation::bundle(&assets))
            .observe(on_drop_equipment::<Weapon>)
            .id();
        on_over_observer.watch_entity(id);
        on_out_observer.watch_entity(id);
    });

    commands.spawn(on_over_observer);
    commands.spawn(on_out_observer);

    commands.queue(|world: &mut World| {
        world.trigger(PlayerEquipmentChanged);
    });
}

fn on_over_equipment(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    mut items: Query<&EquipmentEntity>,
    infos: Query<&ItemInfo>,
    assets: Res<ItemAssets>,
) {
    warn!("on_over_equipment({})", trigger.entity());

    let Ok(EquipmentEntity(Some(equipment_entity))) = items.get_mut(trigger.entity()) else {
        return;
    };

    if let Ok(info) = infos.get(*equipment_entity) {
        commands.spawn(InfoPopup {
            image: Some(assets.image_node(info.tile_index)),
            text: info.text.clone(),
            source: trigger.entity(),
            pos: trigger.event().pointer_location.position,
        });
    }
}

fn on_out_equipment(
    trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    // mut locations: Query<&mut BorderColor, With<InventoryLocation>>,
    popups: Query<Entity, With<InfoPopup>>,
) {
    warn!("on_out_equipment({})", trigger.entity());
    // if let Ok(mut border_color) = locations.get_mut(trigger.entity()) {
    //     border_color.0 = Srgba::NONE.into();
    // }
    for entity in &popups {
        commands.entity(entity).despawn_recursive();
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
            // The item drop is an helmet
            commands.queue(EquipItemCommand(item_entity));
        }
    }
}

fn update_equipment<F1, F2>(
    mut locations: Query<(&mut ImageNode, &mut EquipmentEntity), F1>,
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
        .next() // There should be only one result if it matches
        .unwrap_or((None, assets.empty_image_node()));

    for (mut image_node, mut equipment_entity) in &mut locations {
        *image_node = equipment_image_node.clone();
        equipment_entity.0 = entity_option;
    }
}

fn update_equipments(
    _trigger: Trigger<PlayerEquipmentChanged>,
    helmet_locations: Query<
        (&mut ImageNode, &mut EquipmentEntity),
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
        (&mut ImageNode, &mut EquipmentEntity),
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
        (&mut ImageNode, &mut EquipmentEntity),
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
        (&mut ImageNode, &mut EquipmentEntity),
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
        (&mut ImageNode, &mut EquipmentEntity),
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
    update_equipment(helmet_locations, helmets, *player, &assets);
    update_equipment(body_armour_locations, body_armours, *player, &assets);
    update_equipment(boots_locations, boots, *player, &assets);
    update_equipment(amulet_locations, amulets, *player, &assets);
    update_equipment(weapon_locations, weapons, *player, &assets);
}

pub struct InventoryPanelPlugin;

impl Plugin for InventoryPanelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EquipmentEntity>()
            .add_observer(spawn_panel_content)
            .add_observer(update_equipments);
    }
}
