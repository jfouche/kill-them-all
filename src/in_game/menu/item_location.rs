use super::dnd::{DndCursor, DraggedEntity};
use crate::{
    components::item::{Item, ItemAssets, ItemEntity, ItemImage, ItemInfo, ItemLocation},
    schedule::GameRunningSet,
};
use bevy::{color::palettes::css, ecs::query::QueryFilter, prelude::*};
use std::marker::PhantomData;

pub struct ItemImagePlugin;

impl Plugin for ItemImagePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ItemEntity>()
            .add_systems(Update, update_image.in_set(GameRunningSet::EntityUpdate))
            .add_observer(create_image_location);
    }
}

fn create_image_location(
    trigger: Trigger<OnAdd, ItemLocation>,
    mut commands: Commands,
    item_entities: Query<&ItemEntity, With<ItemLocation>>,
    item_infos: Query<&ItemInfo, With<Item>>,
    assets: Res<ItemAssets>,
) {
    if let Ok(item_entity) = item_entities.get(trigger.entity()) {
        let image_node = match item_entity.0 {
            Some(entity) => item_infos
                .get(entity)
                .ok()
                .map(|info| assets.image_node(info.tile_index))
                .unwrap_or(assets.empty_image_node()),
            None => assets.empty_image_node(),
        };
        commands
            .entity(trigger.entity())
            .with_child((ItemImage, image_node));
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
                    .unwrap_or_else(|| assets.empty_image_node())
            })
            .unwrap_or_else(|| assets.empty_image_node());

        for child in children.iter() {
            if let Ok(mut image_node) = images.get_mut(*child) {
                *image_node = new_image_node.clone();
            }
        }
    }
}

pub struct ShowBorderOnDrag<F = With<Item>> {
    on_enter: Observer,
    on_leave: Observer,
    _phantom: PhantomData<F>,
}

impl<F> ShowBorderOnDrag<F>
where
    F: QueryFilter + 'static,
{
    pub fn new() -> Self {
        Self {
            on_enter: Observer::new(show_borders_on_drag_enter_item::<F>),
            on_leave: Observer::new(hide_borders_on_drag_leave_item),
            _phantom: PhantomData::<F>,
        }
    }

    pub fn watch_entity(&mut self, entity: Entity) {
        self.on_enter.watch_entity(entity);
        self.on_leave.watch_entity(entity);
    }

    pub fn spawn(self, commands: &mut Commands) {
        commands.spawn(self.on_enter);
        commands.spawn(self.on_leave);
    }
}

fn show_borders_on_drag_enter_item<F>(
    trigger: Trigger<Pointer<DragEnter>>,
    mut colors: Query<&mut BackgroundColor, With<ItemLocation>>,
    items: Query<(), F>,
    cursor: Single<&DraggedEntity, With<DndCursor>>,
) where
    F: QueryFilter,
{
    if let Some(item_entity) = ***cursor {
        warn!("show_borders_on_drag_enter_item({})", trigger.entity());
        if items.get(item_entity).is_ok() {
            if let Ok(mut color) = colors.get_mut(trigger.entity()) {
                color.0 = css::DARK_ORANGE.into();
            }
        }
    }
}

fn hide_borders_on_drag_leave_item(
    trigger: Trigger<Pointer<DragLeave>>,
    mut colors: Query<&mut BackgroundColor, With<ItemLocation>>,
) {
    if let Ok(mut color) = colors.get_mut(trigger.entity()) {
        warn!("hide_borders_on_drag_leave_item({})", trigger.entity());
        color.0 = Srgba::NONE.into();
    };
}

pub struct ItemLocationDragObservers(Vec<Observer>);

impl ItemLocationDragObservers {
    pub fn new() -> Self {
        Self(vec![
            Observer::new(on_drag_start_item),
            Observer::new(on_drag_end_item),
        ])
    }

    pub fn watch_entity(&mut self, entity: Entity) {
        self.0.iter_mut().for_each(|o| {
            o.watch_entity(entity);
        });
    }

    pub fn spawn(self, commands: &mut Commands) {
        commands.spawn_batch(self.0);
    }
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

fn on_drag_end_item(
    trigger: Trigger<Pointer<DragEnd>>,
    cursor: Single<(&mut DraggedEntity, &mut ImageNode), With<DndCursor>>,
) {
    warn!("on_drag_end_item({})", trigger.entity());
    let (mut dragged_entity, mut cursor_image) = cursor.into_inner();
    **dragged_entity = None;
    *cursor_image = ImageNode::default();
}
