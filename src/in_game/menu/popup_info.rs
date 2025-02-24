use crate::{
    components::item::{ItemAssets, ItemEntity, ItemInfo},
    ui::popup::Popup,
};
use bevy::prelude::*;

pub struct InfoPopupPlugin;

impl Plugin for InfoPopupPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ItemEntity>();
    }
}

/// The popup itself
#[derive(Component)]
#[require(
    Name(|| Name::new("InfoPopup")),
    Popup,
    Node,
    ZIndex(|| ZIndex(1))
)]
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
///
/// It needs an [ItemEntity] component.
pub struct SpawnInfoPopupObservers {
    spawn: Observer,
    despawn: Observer,
}

impl SpawnInfoPopupObservers {
    pub fn new() -> Self {
        Self {
            spawn: Observer::new(spawn_popup_info_on_over_item),
            despawn: Observer::new(despawn_popup_info_on_out_item),
        }
    }

    pub fn watch_entity(&mut self, entity: Entity) {
        self.spawn.watch_entity(entity);
        self.despawn.watch_entity(entity);
    }

    pub fn spawn(self, commands: &mut Commands) {
        commands.spawn(self.spawn);
        commands.spawn(self.despawn);
    }
}

fn spawn_popup_info_on_over_item(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    mut item_entities: Query<&ItemEntity>,
    items: Query<&ItemInfo>,
    assets: Res<ItemAssets>,
) {
    warn!("spawn_popup_info_on_over_item({})", trigger.entity());
    if let Ok(ItemEntity(Some(item_entity))) = item_entities.get_mut(trigger.entity()) {
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
    };
}

fn despawn_popup_info_on_out_item(
    trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    popups: Query<Entity, With<InfoPopup>>,
) {
    warn!("despawn_popup_info_on_out_item({})", trigger.entity());
    for entity in &popups {
        commands.entity(entity).despawn_recursive();
    }
}
