use super::dnd::{DndCursor, DraggedEntity};
use crate::{
    components::item::{Item, ItemAssets, ItemEntity, ItemImage, ItemInfo, ItemLocation},
    schedule::GameRunningSet,
};
use bevy::{color::palettes::css, ecs::query::QueryFilter, prelude::*};
use std::marker::PhantomData;

pub struct ItemLocationPlugin;

impl Plugin for ItemLocationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ItemEntity>()
            .add_systems(Update, update_image.in_set(GameRunningSet::EntityUpdate))
            .add_observer(create_image_location)
            .add_observer(on_drag_start_item)
            .add_observer(on_drag_end_item);
    }
}

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

#[derive(Deref, DerefMut)]
pub struct ShowBorderOnDrag<F = With<Item>>(#[deref] pub Vec<Observer>, PhantomData<F>);

impl<F> Default for ShowBorderOnDrag<F>
where
    F: QueryFilter + 'static,
{
    fn default() -> Self {
        Self(Self::observers(), PhantomData)
    }
}

impl<F> ShowBorderOnDrag<F>
where
    F: QueryFilter + 'static,
{
    pub fn observers() -> Vec<Observer> {
        vec![
            Observer::new(show_borders_on_drag_enter_item::<F>),
            Observer::new(hide_borders_on_drag_leave_item),
        ]
    }
}

fn show_borders_on_drag_enter_item<F>(
    mut trigger: Trigger<Pointer<DragEnter>>,
    mut colors: Query<&mut BackgroundColor, With<ItemLocation>>,
    items: Query<(), F>,
    cursor: Single<&DraggedEntity, With<DndCursor>>,
) where
    F: QueryFilter,
{
    if let Some(item_entity) = ***cursor {
        // info!("show_borders_on_drag_enter_item({})", trigger.entity());
        if items.get(item_entity).is_ok() {
            if let Ok(mut color) = colors.get_mut(trigger.target()) {
                color.0 = css::DARK_ORANGE.into();
            }
        }
    }
    trigger.propagate(false);
}

fn hide_borders_on_drag_leave_item(
    mut trigger: Trigger<Pointer<DragLeave>>,
    mut colors: Query<&mut BackgroundColor, With<ItemLocation>>,
) {
    if let Ok(mut color) = colors.get_mut(trigger.target()) {
        // info!("hide_borders_on_drag_leave_item({})", trigger.entity());
        color.0 = Srgba::NONE.into();
    }
    trigger.propagate(false);
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
