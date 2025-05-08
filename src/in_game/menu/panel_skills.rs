use super::dnd::{DndCursor, DraggedEntity};
use crate::{
    components::{
        inventory::PlayerEquipmentChanged,
        player::{EquipSkillBookEvent, PlayerAction},
        skills::{SkillBook, SkillBookLocation},
    },
    utils::observers::VecObserversExt,
};
use bevy::prelude::*;

#[derive(Component)]
#[require(
    Name::new("SkillsPanel"),
    Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(5.)),
        ..Default::default()
    }
)]
pub struct SkillsPanel;

pub struct SkillsPanelPlugin;

impl Plugin for SkillsPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(create_panel);
    }
}

fn create_panel(trigger: Trigger<OnAdd, SkillsPanel>, mut commands: Commands) {
    let mut observers = vec![Observer::new(on_drop_item)];

    commands.entity(trigger.target()).with_children(|panel| {
        panel.spawn(Text::new("A:"));
        let entity = panel.spawn((PlayerAction::Skill1, SkillBookLocation)).id();
        observers.watch_entity(entity);

        panel.spawn(Text::new("Z:"));
        let entity = panel.spawn((PlayerAction::Skill2, SkillBookLocation)).id();
        observers.watch_entity(entity);

        panel.spawn(Text::new("E:"));
        let entity = panel.spawn((PlayerAction::Skill3, SkillBookLocation)).id();
        observers.watch_entity(entity);

        panel.spawn(Text::new("R:"));
        let entity = panel.spawn((PlayerAction::Skill4, SkillBookLocation)).id();
        observers.watch_entity(entity);
    });
    commands.spawn_batch(observers);

    // to force to init the update
    commands.trigger(PlayerEquipmentChanged);
}

fn on_drop_item(
    trigger: Trigger<Pointer<DragDrop>>,
    mut commands: Commands,
    locations: Query<&PlayerAction, With<SkillBookLocation>>,
    cursor: Single<&DraggedEntity, With<DndCursor>>,
    books: Query<(), With<SkillBook>>,
) {
    if let Some(item_entity) = ***cursor {
        if books.get(item_entity).is_ok() {
            if let Ok(action) = locations.get(trigger.target()) {
                // The item dropped is a skill gem
                // TODO: Obviously wrong!
                commands.trigger(EquipSkillBookEvent {
                    action: *action,
                    book_entity: item_entity,
                });
            }
        }
    }
}
