use crate::ui::Popup;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

#[derive(Component, Default, Clone)]
#[component(on_add = create_popup)]
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
pub struct InfoPopup {
    image: Option<Handle<Image>>,
    atlas: Option<TextureAtlas>,
    text: Option<String>,
    pos: Vec2,
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

    pub fn at(mut self, pos: Vec2) -> Self {
        self.pos = pos;
        self
    }
}

fn create_popup(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    let info = world
        .get::<InfoPopup>(entity)
        .expect("InfoPopup added")
        .clone();
    world.commands().entity(entity).with_children(|parent| {
        if let Some(image) = info.image {
            if let Some(atlas) = info.atlas {
                parent.spawn(ImageNode::from_atlas_image(image, atlas));
            } else {
                parent.spawn(ImageNode::new(image));
            }
        }
        if let Some(text) = info.text {
            parent.spawn((Text(text), TextFont::from_font_size(12.)));
        }
    });
    let mut node = world.get_mut::<Node>(entity).expect("Node");
    node.left = Val::Px(info.pos.x + 5.);
    node.top = Val::Px(info.pos.y - 20.);
}

pub struct InfoPopupPlugin;

impl Plugin for InfoPopupPlugin {
    fn build(&self, app: &mut App) {}
}
