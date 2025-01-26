use crate::{
    components::inventory::{DropItemCommand, EquipItemCommand},
    ui::{button::TextButton, popup::Popup, HSizer},
};
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

/// Show a popup that allow the user to use an [Orb]
/// when the user click on the attached entity.
#[derive(Component, Clone)]
#[component(on_add = init_mouse_over)]
pub struct ShowOrbActionsOnClick {
    pub text: String,
    pub image: Option<ImageNode>,
    pub item: Entity,
}

fn init_mouse_over(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    world.commands().entity(entity).observe(spawn_popup);
}

fn spawn_popup(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    contents: Query<&ShowOrbActionsOnClick>,
    popups: Query<Entity, With<OrbPopup>>,
) {
    if let Ok(content) = contents.get(trigger.entity()) {
        // Allow only one OrbPopup
        for entity in &popups {
            commands.entity(entity).despawn_recursive();
        }
        commands.spawn(OrbPopup).with_children(|parent| {
            let popup_entity = parent.parent_entity();
            if let Some(image_node) = &content.image {
                parent.spawn(image_node.clone());
            }
            parent.spawn((Text(content.text.clone()), TextFont::from_font_size(12.)));
            parent.spawn(HSizer).with_children(|hsizer| {
                hsizer
                    .spawn((
                        TextButton::small("Use"),
                        ItemAction::Use(ItemAndPopup {
                            item: content.item,
                            popup: popup_entity,
                        }),
                    ))
                    .observe(item_action);
                hsizer
                    .spawn((
                        TextButton::small("Drop"),
                        ItemAction::Drop(ItemAndPopup {
                            item: content.item,
                            popup: popup_entity,
                        }),
                    ))
                    .observe(item_action);
                hsizer
                    .spawn((
                        TextButton::small("Cancel"),
                        ItemAction::DespawnPopup(popup_entity),
                    ))
                    .observe(item_action);
            });
        });
    }
}

/// The popup
#[derive(Component, Clone)]
#[require(
    Name(|| Name::new("OrbPopup")),
    Popup,
    Node(|| Node {
        margin: UiRect::all(Val::Auto),
        padding: UiRect::all(Val::Px(5.)),
        ..Popup::default_node()
    }),
    ZIndex(|| ZIndex(1))
)]
struct OrbPopup;

#[derive(Component)]
enum ItemAction {
    Use(ItemAndPopup),
    Drop(ItemAndPopup),
    DespawnPopup(Entity),
}

struct ItemAndPopup {
    item: Entity,
    popup: Entity,
}

fn item_action(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    actions: Query<&ItemAction>,
) {
    if let Ok(action) = actions.get(trigger.entity()) {
        match &action {
            &ItemAction::Use(entities) => {
                commands.queue(EquipItemCommand(entities.item));
                commands.entity(entities.popup).despawn_recursive();
            }
            &ItemAction::Drop(entities) => {
                commands.queue(DropItemCommand(entities.item));
                commands.entity(entities.popup).despawn_recursive();
            }
            ItemAction::DespawnPopup(entity) => {
                commands.entity(*entity).despawn_recursive();
            }
        }
    }
}
