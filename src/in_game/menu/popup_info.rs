use crate::{
    components::item::{ItemAssets, ItemEntity, ItemInfo},
    ui::popup::Popup,
};
use bevy::prelude::*;

/// The popup itself
#[derive(Component)]
#[require(Name::new("InfoPopup"), Popup, Node, ZIndex(1))]
struct InfoPopup;

impl InfoPopup {
    fn node(pos: Vec2) -> Node {
        Node {
            max_width: Val::Px(180.),
            margin: UiRect::all(Val::Px(0.)),
            padding: UiRect::all(Val::Px(5.)),
            left: Val::Px(pos.x - 60.),
            top: Val::Px(pos.y - 130.),
            ..Popup::default_node()
        }
    }
}

/// Observers that shows an info popup when overing an Item.
pub struct SpawnInfoPopupObservers(/* pub Vec<Observer> */);

// impl Default for SpawnInfoPopupObservers {
//     fn default() -> Self {
//         Self(Self::observers())
//     }
// }

impl SpawnInfoPopupObservers {
    pub fn observers() -> Vec<Observer> {
        vec![
            Observer::new(spawn_popup_info_on_over_item),
            Observer::new(despawn_popup_info_on_out_item),
        ]
    }
}

fn spawn_popup_info_on_over_item(
    mut trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    mut item_entities: Query<&ItemEntity>,
    items: Query<&ItemInfo>,
    assets: Res<ItemAssets>,
) {
    if let Ok(ItemEntity(Some(item_entity))) = item_entities.get_mut(trigger.target()) {
        // info!("spawn_popup_info_on_over_item({})", trigger.entity());
        if let Ok(info) = items.get(*item_entity) {
            commands
                .spawn((
                    InfoPopup,
                    InfoPopup::node(trigger.event().pointer_location.position),
                ))
                .with_children(|popup| {
                    popup.spawn(assets.image_node(info.tile_index));
                    popup.spawn((Text(info.text.clone()), TextFont::from_font_size(12.)));
                });
        }
        // TODO: https://github.com/bevyengine/bevy/blob/release-0.15.3/crates/bevy_picking/src/events.rs#L353
        trigger.propagate(false);
    }
}

fn despawn_popup_info_on_out_item(
    mut trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    popups: Query<Entity, With<InfoPopup>>,
) {
    for entity in &popups {
        // info!("despawn_popup_info_on_out_item({})", trigger.entity());
        commands.entity(entity).despawn();
    }
    trigger.propagate(false);
}
