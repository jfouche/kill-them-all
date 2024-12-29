use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

#[derive(Component, Default)]
#[component(on_add = create_popup)]
#[require(
    Node(Popup::default_node),
    BackgroundColor(|| BackgroundColor(Color::srgb(0.25, 0.25, 0.25))),
    BorderColor(|| BorderColor(Color::BLACK)),
)]
pub struct Popup {
    title: Option<String>,
}

impl Popup {
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[inline]
    pub fn default_node() -> Node {
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            width: Val::Percent(40.0),
            margin: UiRect::all(Val::Auto),
            padding: UiRect::bottom(Val::Px(7.0)),
            ..Default::default()
        }
    }
}

fn create_popup(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    world.commands().queue(CreatePopupCommand(entity));
}

struct CreatePopupCommand(Entity);

impl Command for CreatePopupCommand {
    fn apply(self, world: &mut World) {
        let title = world
            .get::<Popup>(self.0)
            .expect("Added Popup")
            .title
            .clone();
        if let Some(title) = title {
            world.commands().entity(self.0).with_children(|parent| {
                parent.spawn(PopupTitleBar).with_children(|bar| {
                    bar.spawn((PopupTitle, Text(title)));
                });
            });
        }
    }
}

#[derive(Component)]
#[require(
    Name(|| Name::new("PopupTitleBar")),
    Node(|| Node {
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        ..Default::default()
    }),
    BackgroundColor(|| BackgroundColor(Color::srgb(0.1, 0.1, 0.1))),
)]
struct PopupTitleBar;

#[derive(Component)]
#[require(
    Text,
    TextFont(|| TextFont::from_font_size(32.)),
    TextColor(|| TextColor(Color::srgb(0.72, 0.72, 0.72)))
)]
struct PopupTitle;
