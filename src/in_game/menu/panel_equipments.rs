use super::popup_info::ShowPopupOnMouseOver;
use crate::components::*;
use bevy::{ecs::query::QueryFilter, prelude::*};

///
/// Inventory panel
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

fn spawn_equipment<T>(panel: &mut ChildBuilder, info: ItemInfo, assets: &EquipmentAssets)
where
    T: Component + EquipmentPos,
{
    let pos = T::pos();
    panel.spawn((
        InventoryBox,
        Node {
            left: Val::Px(pos.x),
            top: Val::Px(pos.y),
            ..default_inventory_box_node()
        },
        assets.image_node(info.tile_index),
        info.clone(),
        ShowPopupOnMouseOver {
            text: info.text,
            image: Some(assets.image_node(info.tile_index)),
        },
    ));
}

fn show_all_equipments(
    panel: &mut ChildBuilder,
    helmets: Query<(&ItemInfo, &Parent), With<Helmet>>,
    body_armours: Query<(&ItemInfo, &Parent), With<BodyArmour>>,
    boots: Query<(&ItemInfo, &Parent), With<Boots>>,
    amulets: Query<(&ItemInfo, &Parent), With<Amulet>>,
    weapons: Query<(&ItemInfo, &Parent), With<Weapon>>,
    player: Entity,
    assets: &EquipmentAssets,
) {
    if let Some(info) = get_equipment(helmets, player) {
        spawn_equipment::<Helmet>(panel, info, assets);
    }
    if let Some(info) = get_equipment(body_armours, player) {
        spawn_equipment::<BodyArmour>(panel, info, assets);
    }
    if let Some(info) = get_equipment(boots, player) {
        spawn_equipment::<Boots>(panel, info, assets);
    }
    if let Some(info) = get_equipment(amulets, player) {
        spawn_equipment::<Amulet>(panel, info, assets);
    }
    if let Some(info) = get_equipment(weapons, player) {
        spawn_equipment::<Weapon>(panel, info, assets);
    }
}

fn get_equipment<F>(query: Query<(&ItemInfo, &Parent), F>, player: Entity) -> Option<ItemInfo>
where
    F: QueryFilter,
{
    query
        .iter()
        .filter(|(_info, parent)| ***parent == player)
        .map(|(info, _)| info.clone())
        .next()
}

fn show_equipments(
    trigger: Trigger<OnAdd, EquipmentsPanel>,
    mut commands: Commands,
    helmets: Query<(&ItemInfo, &Parent), With<Helmet>>,
    body_armours: Query<(&ItemInfo, &Parent), With<BodyArmour>>,
    boots: Query<(&ItemInfo, &Parent), With<Boots>>,
    amulets: Query<(&ItemInfo, &Parent), With<Amulet>>,
    weapons: Query<(&ItemInfo, &Parent), With<Weapon>>,
    player: Single<Entity, With<Player>>,
    assets: Res<EquipmentAssets>,
) {
    commands.entity(trigger.entity()).with_children(|panel| {
        show_all_equipments(
            panel,
            helmets,
            body_armours,
            boots,
            amulets,
            weapons,
            *player,
            &assets,
        );
    });
}

fn update_equipments(
    _trigger: Trigger<PlayerEquipmentChanged>,
    mut commands: Commands,
    panels: Query<Entity, With<EquipmentsPanel>>,
    mut helmets: Query<(&ItemInfo, &Parent), With<Helmet>>,
    mut body_armours: Query<(&ItemInfo, &Parent), With<BodyArmour>>,
    mut boots: Query<(&ItemInfo, &Parent), With<Boots>>,
    mut amulets: Query<(&ItemInfo, &Parent), With<Amulet>>,
    mut weapons: Query<(&ItemInfo, &Parent), With<Weapon>>,
    player: Single<Entity, With<Player>>,
    assets: Res<EquipmentAssets>,
) {
    for panel in &panels {
        commands.entity(panel).despawn_descendants();
        commands.entity(panel).with_children(|panel| {
            show_all_equipments(
                panel,
                helmets.reborrow(),
                body_armours.reborrow(),
                boots.reborrow(),
                amulets.reborrow(),
                weapons.reborrow(),
                *player,
                &assets,
            );
        });
    }
}

pub struct InventoryPanelPlugin;

impl Plugin for InventoryPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(show_equipments)
            .add_observer(update_equipments);
    }
}
