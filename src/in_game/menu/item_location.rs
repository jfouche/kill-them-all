use crate::{
    components::{
        equipment::{Amulet, BodyArmour, Boots, Helmet, Weapon},
        item::{
            Item, ItemAssets, ItemEntity, ItemImage, ItemInfo, ItemLocation, ItemLocationAccept,
            ItemLocationAcceptAll,
        },
        skills::SkillBook,
    },
    in_game::dnd::{DndCursor, DraggedEntity},
    schedule::GameRunningSet,
};
use bevy::{color::palettes::css, prelude::*};

pub struct ItemLocationPlugin;

impl Plugin for ItemLocationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ItemEntity>()
            .add_systems(Update, update_image.in_set(GameRunningSet::EntityUpdate))
            .add_observer(create_image_location)
            .add_observer(on_drag_start_item)
            .add_observer(on_drag_end_item)
            .add_observer(show_location_borders)
            .add_observer(show_location_borders_filtered::<Helmet>)
            .add_observer(show_location_borders_filtered::<BodyArmour>)
            .add_observer(show_location_borders_filtered::<Boots>)
            .add_observer(show_location_borders_filtered::<Weapon>)
            .add_observer(show_location_borders_filtered::<Amulet>)
            .add_observer(show_location_borders_filtered::<SkillBook>)
            .add_observer(hide_location_borders);
    }
}

const BORDER_COLOR: Color = Color::Srgba(css::DARK_ORANGE);

fn create_image_location(
    trigger: Trigger<OnAdd, ItemLocation>,
    mut commands: Commands,
    item_entities: Query<&ItemEntity, With<ItemLocation>>,
    item_infos: Query<&ItemInfo, With<Item>>,
    assets: Res<ItemAssets>,
) {
    if let Ok(item_entity) = item_entities.get(trigger.target()) {
        let image_node = match item_entity.0 {
            Some(entity) => item_infos
                .get(entity)
                .ok()
                .map(|info| assets.image_node(info.tile_index))
                .unwrap_or(assets.empty_image_node()),
            None => assets.empty_image_node(),
        };
        commands.spawn((ItemImage, image_node, ChildOf(trigger.target())));
    }
}

fn update_image(
    locations: Query<(&ItemEntity, &Children), Changed<ItemEntity>>,
    item_infos: Query<&ItemInfo, With<Item>>,
    mut images: Query<&mut ImageNode, With<ItemImage>>,
    assets: Res<ItemAssets>,
) {
    for (item_entity, children) in &locations {
        let new_image_node = item_entity
            .0
            .map(|entity| {
                item_infos
                    .get(entity)
                    .ok()
                    .map(|info| assets.image_node(info.tile_index))
            })
            .flatten()
            .unwrap_or_else(|| assets.empty_image_node());

        for child in children.iter() {
            if let Ok(mut image_node) = images.get_mut(child) {
                *image_node = new_image_node.clone();
            }
        }
    }
}

fn show_location_borders(
    trigger: Trigger<Pointer<DragEnter>>,
    mut colors: Query<&mut BackgroundColor, (With<ItemLocation>, With<ItemLocationAcceptAll>)>,
    items: Query<(), With<Item>>,
    cursor: Single<&DraggedEntity, With<DndCursor>>,
) {
    if let Some(item_entity) = ***cursor {
        if items.contains(item_entity) {
            if let Ok(mut color) = colors.get_mut(trigger.target()) {
                color.0 = BORDER_COLOR.into();
            }
        }
    }
}

fn show_location_borders_filtered<T>(
    trigger: Trigger<Pointer<DragEnter>>,
    mut colors: Query<&mut BackgroundColor, (With<ItemLocation>, With<ItemLocationAccept<T>>)>,
    items: Query<(), (With<Item>, With<T>)>,
    cursor: Single<&DraggedEntity, With<DndCursor>>,
) where
    T: Component,
{
    if let Some(item_entity) = ***cursor {
        if items.contains(item_entity) {
            if let Ok(mut color) = colors.get_mut(trigger.target()) {
                color.0 = BORDER_COLOR.into();
            }
        }
    }
}

fn hide_location_borders(
    trigger: Trigger<Pointer<DragLeave>>,
    mut colors: Query<&mut BackgroundColor, With<ItemLocation>>,
) {
    if let Ok(mut color) = colors.get_mut(trigger.target()) {
        color.0 = Srgba::NONE.into();
    }
}

fn on_drag_start_item(
    trigger: Trigger<Pointer<DragStart>>,
    locations: Query<&ItemEntity, With<ItemLocation>>,
    infos: Query<&ItemInfo>,
    cursor: Single<(&mut DraggedEntity, &mut ImageNode), With<DndCursor>>,
    assets: Res<ItemAssets>,
) {
    if let Ok(ItemEntity(Some(item))) = locations.get(trigger.target()) {
        if let Ok(info) = infos.get(*item) {
            let (mut dragged_entity, mut cursor_image) = cursor.into_inner();
            **dragged_entity = Some(*item);
            *cursor_image = assets.image_node(info.tile_index);
        }
    }
}

fn on_drag_end_item(
    _trigger: Trigger<Pointer<DragEnd>>,
    cursor: Single<(&mut DraggedEntity, &mut ImageNode), With<DndCursor>>,
) {
    let (mut dragged_entity, mut cursor_image) = cursor.into_inner();
    **dragged_entity = None;
    *cursor_image = ImageNode::default();
}
