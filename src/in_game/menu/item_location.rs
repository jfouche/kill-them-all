use crate::{
    components::item::{Item, ItemAssets, ItemEntity, ItemImage, ItemInfo, ItemLocation},
    schedule::GameRunningSet,
};
use bevy::prelude::*;

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
