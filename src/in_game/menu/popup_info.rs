use crate::{
    components::item::{ItemAssets, ItemDescription, ItemEntity, ItemTileIndex, ItemTitle},
    ui::popup::Popup,
};
use bevy::prelude::*;

pub struct PopupInfoPlugin;

impl Plugin for PopupInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_popup_info_on_over_item)
            .add_observer(despawn_popup_info_on_out_item);
    }
}

/// The popup itself
#[derive(Component)]
struct InfoPopup;

fn info_popup(pos: Vec2, img_node: ImageNode, title: String, description: String) -> impl Bundle {
    (
        InfoPopup,
        Name::new("InfoPopup"),
        Popup,
        Node {
            max_width: Val::Px(180.),
            margin: UiRect::all(Val::Px(0.)),
            padding: UiRect::all(Val::Px(5.)),
            left: Val::Px(pos.x - 60.),
            top: Val::Px(pos.y - 130.),
            ..Popup::default_node()
        },
        ZIndex(1),
        children![
            (
                Text(title),
                TextFont::from_font_size(12.),
                TextLayout::new_with_justify(JustifyText::Center)
            ),
            img_node,
            (Text(description), TextFont::from_font_size(12.))
        ],
    )
}

fn spawn_popup_info_on_over_item(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    mut item_entities: Query<&ItemEntity>,
    items: Query<(&ItemTitle, &ItemDescription, &ItemTileIndex)>,
    assets: Res<ItemAssets>,
) {
    if let Ok(ItemEntity(Some(item_entity))) = item_entities.get_mut(trigger.target()) {
        if let Ok((title, description, tile_index)) = items.get(*item_entity) {
            let pos = trigger.pointer_location.position;
            let img = assets.image_node(tile_index.0);
            commands.spawn(info_popup(pos, img, title.0.clone(), description.0.clone()));
        }
    }
}

fn despawn_popup_info_on_out_item(
    _trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    popups: Query<Entity, With<InfoPopup>>,
) {
    for entity in &popups {
        // TODO: Why entity might be already despaned ?
        commands.entity(entity).try_despawn();
    }
}
