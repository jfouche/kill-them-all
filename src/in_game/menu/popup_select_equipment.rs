use crate::{components::*, ui::*};
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

/// Show a popup that allow the user to equip or dismiss an equipment
/// when the user click on the attached entity.
#[derive(Component, Clone)]
#[component(on_add = init)]
pub struct ShowEquipmentActionsOnMouseOver {
    pub text: String,
    pub image: Option<ImageNode>,
    pub item: Entity,
}

fn init(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    world.commands().entity(entity).observe(spawn_popup);
}

fn spawn_popup(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    contents: Query<&ShowEquipmentActionsOnMouseOver>,
) {
    if let Ok(content) = contents.get(trigger.entity()) {
        commands.spawn(EquipmentPopup {
            content: content.clone(),
        });
    }
}

/// The popup
#[derive(Component, Clone)]
#[component(on_add = create_popup)]
#[require(
    Name(|| Name::new("EquipmentPopup")),
    Popup,
    Node(|| Node {
        margin: UiRect::all(Val::Auto),
        padding: UiRect::all(Val::Px(5.)),
        ..Popup::default_node()
    }),
    ZIndex(|| ZIndex(1))
)]
struct EquipmentPopup {
    content: ShowEquipmentActionsOnMouseOver,
}

fn create_popup(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    world.commands().queue(CreatePopupCommand(entity));
}

struct CreatePopupCommand(Entity);

impl Command for CreatePopupCommand {
    fn apply(self, world: &mut World) {
        let popup = world
            .get::<EquipmentPopup>(self.0)
            .expect("EquipmentPopup added")
            .clone();
        world.entity_mut(self.0).with_children(|parent| {
            if let Some(image_node) = popup.content.image {
                parent.spawn(image_node);
            }
            parent.spawn((Text(popup.content.text), TextFont::from_font_size(12.)));
            parent.spawn(HSizer).with_children(|hsizer| {
                hsizer
                    .spawn((
                        TextButton::small("Equip"),
                        ItemAction::Equip(ItemAndPopup {
                            item: popup.content.item,
                            popup: self.0,
                        }),
                    ))
                    .observe(item_action);
                hsizer
                    .spawn((
                        TextButton::small("Drop"),
                        ItemAction::Drop(ItemAndPopup {
                            item: popup.content.item,
                            popup: self.0,
                        }),
                    ))
                    .observe(item_action);
                hsizer
                    .spawn((
                        TextButton::small("Cancel"),
                        ItemAction::DespawnPopup(self.0),
                    ))
                    .observe(item_action);
            });
        });
    }
}

#[derive(Component)]
enum ItemAction {
    Equip(ItemAndPopup),
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
            &ItemAction::Equip(entities) => {
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
