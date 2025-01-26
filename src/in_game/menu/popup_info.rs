use crate::ui::popup::Popup;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

/// Component to add to allow a popup when overing the entity
#[derive(Component, Default, Clone)]
#[component(on_add = init_show_popup_on_mouse_over)]
pub struct ShowPopupOnMouseOver {
    pub text: String,
    pub image: Option<ImageNode>,
}

fn init_show_popup_on_mouse_over(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    world
        .commands()
        .entity(entity)
        .observe(spawn_popup)
        .observe(despawn_popup_on_out)
        .observe(despawn_popup_on_click)
        .observe(despawn_popup_on_removed);
}

/// System to add to an entity observer to spawn the [InfoPopup]
/// when overing the observed entity
fn spawn_popup(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    infos: Query<&ShowPopupOnMouseOver>,
    popups: Query<&InfoPopup>,
) {
    if let Ok(info) = infos.get(trigger.entity()) {
        if !popups.iter().any(|popup| popup.source == trigger.entity()) {
            commands.spawn(InfoPopup {
                info: info.clone(),
                source: trigger.entity(),
                pos: trigger.event().pointer_location.position,
            });
        }
    }
}

/// System to add to an entity observer to despawn the [InfoPopup]
/// when overing out the observed entity
fn despawn_popup_on_out(
    _trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    popups: Query<Entity, With<InfoPopup>>,
) {
    if let Ok(entity) = popups.get_single() {
        commands.entity(entity).despawn_recursive()
    }
}

fn despawn_popup_on_click(
    _trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    popups: Query<Entity, With<InfoPopup>>,
) {
    if let Ok(entity) = popups.get_single() {
        commands.entity(entity).despawn_recursive()
    }
}

fn despawn_popup_on_removed(
    _trigger: Trigger<OnRemove, ShowPopupOnMouseOver>,
    mut commands: Commands,
    popups: Query<Entity, With<InfoPopup>>,
) {
    if let Ok(entity) = popups.get_single() {
        commands.entity(entity).despawn_recursive()
    }
}

/// The popup itself
#[derive(Component, Clone)]
#[component(on_add = create_popup)]
#[require(
    Name(|| Name::new("InfoPopup")),
    Popup,
    Node(|| Node {
        max_width: Val::Px(180.),
        margin: UiRect::all(Val::Px(0.)),
        padding: UiRect::all(Val::Px(5.)),
        ..Popup::default_node()
    }),
    ZIndex(|| ZIndex(1))
)]
struct InfoPopup {
    info: ShowPopupOnMouseOver,
    source: Entity,
    pos: Vec2,
}

fn create_popup(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    world.commands().queue(CreatePopupCommand(entity));
}

struct CreatePopupCommand(Entity);

impl Command for CreatePopupCommand {
    fn apply(self, world: &mut World) {
        let popup = world
            .get::<InfoPopup>(self.0)
            .expect("InfoPopup added")
            .clone();
        world.entity_mut(self.0).with_children(|parent| {
            if let Some(image_node) = popup.info.image {
                parent.spawn(image_node);
            }
            parent.spawn((Text(popup.info.text), TextFont::from_font_size(12.)));
        });
        let mut node = world.get_mut::<Node>(self.0).expect("Node");
        node.left = Val::Px(popup.pos.x - 60.);
        node.top = Val::Px(popup.pos.y - 130.);
    }
}
