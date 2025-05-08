use crate::{
    components::item::{ItemAssets, ItemEntity, ItemInfo},
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

impl InfoPopup {
    fn bundle(pos: Vec2, img_node: ImageNode, text: String) -> impl Bundle {
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
            children![img_node, (Text(text), TextFont::from_font_size(12.))],
        )
    }
}

fn spawn_popup_info_on_over_item(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    mut item_entities: Query<&ItemEntity>,
    items: Query<&ItemInfo>,
    assets: Res<ItemAssets>,
) {
    if let Ok(ItemEntity(Some(item_entity))) = item_entities.get_mut(trigger.target()) {
        if let Ok(info) = items.get(*item_entity) {
            let pos = trigger.pointer_location.position;
            let img = assets.image_node(info.tile_index);
            commands.spawn(InfoPopup::bundle(pos, img, info.text.clone()));
        }
    }
}

fn despawn_popup_info_on_out_item(
    _trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    popups: Query<Entity, With<InfoPopup>>,
) {
    for entity in &popups {
        commands.entity(entity).despawn();
    }
}
