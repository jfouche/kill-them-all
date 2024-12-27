use crate::ui::Popup;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

#[derive(Component, Default, Clone)]
#[component(on_add = create_popup_window)]
#[require(
    Popup,
    Node(|| Node {
        width: Val::Auto,
        height: Val::Auto,
        margin: UiRect::all(Val::Px(0.)),
        padding: UiRect::all(Val::Px(5.)),
        ..Popup::default_node()
    }),
    ZIndex(|| ZIndex(1))
)]
struct InfoPopupWindow {
    info: InfoPopup,
    pos: Vec2,
}

fn create_popup_window(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    let window = world
        .get::<InfoPopupWindow>(entity)
        .expect("InfoPopupWindow added")
        .clone();
    world.commands().entity(entity).with_children(|parent| {
        if let Some(image) = window.info.image {
            if let Some(atlas) = window.info.atlas {
                parent.spawn(ImageNode::from_atlas_image(image, atlas));
            } else {
                parent.spawn(ImageNode::new(image));
            }
        }
        if let Some(text) = window.info.text {
            parent.spawn((Text(text), TextFont::from_font_size(12.)));
        }
    });
    let mut node = world.get_mut::<Node>(entity).expect("Node");
    node.left = Val::Px(window.pos.x + 5.);
    node.top = Val::Px(window.pos.y - 20.);
}

/// Component to add to allow a popup when overing the entity
#[derive(Component, Default, Clone)]
#[component(on_add = add_observers)]
pub struct InfoPopup {
    image: Option<Handle<Image>>,
    atlas: Option<TextureAtlas>,
    text: Option<String>,
}

impl InfoPopup {
    pub fn new(text: String) -> Self {
        InfoPopup {
            text: Some(text),
            ..Default::default()
        }
    }

    pub fn with_image_atlas(mut self, image: Handle<Image>, atlas: TextureAtlas) -> Self {
        self.image = Some(image);
        self.atlas = Some(atlas);
        self
    }
}

/// System to add to an entity observer to spawn the [InfoPopup]
/// when overing the observed entity
fn spawn_popup(trigger: Trigger<Pointer<Over>>, mut commands: Commands, infos: Query<&InfoPopup>) {
    if let Ok(info) = infos.get(trigger.entity()) {
        commands.spawn(InfoPopupWindow {
            info: info.clone(),
            pos: trigger.event().pointer_location.position,
        });
    }
}

/// System to add to an entity observer to despawn the [InfoPopup]
/// when overing out the observed entity
fn despawn_popup_on_out(
    _trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    popups: Query<Entity, With<InfoPopupWindow>>,
) {
    if let Ok(entity) = popups.get_single() {
        commands.entity(entity).despawn_recursive()
    }
}

fn despawn_popup_on_removed(
    _trigger: Trigger<OnRemove, InfoPopup>,
    mut commands: Commands,
    popups: Query<Entity, With<InfoPopupWindow>>,
) {
    if let Ok(entity) = popups.get_single() {
        commands.entity(entity).despawn_recursive()
    }
}

fn add_observers(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    world
        .commands()
        .entity(entity)
        .observe(spawn_popup)
        .observe(despawn_popup_on_out)
        .observe(despawn_popup_on_removed);
}
