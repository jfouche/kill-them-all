use super::dnd::{DndCursor, DraggedEntity};
use crate::components::{
    inventory::PlayerEquipmentChanged,
    item::ItemLocationAccept,
    player::{EquipSkillBookEvent, PlayerAction},
    skills::{SkillBook, SkillBookLocation},
};
use bevy::prelude::*;

pub struct SkillsPanelPlugin;

impl Plugin for SkillsPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(create_panel).add_observer(on_drop_item);
    }
}

#[derive(Component)]
struct SkillsPanel;

pub fn skills_panel() -> impl Bundle {
    (
        SkillsPanel,
        Name::new("SkillsPanel"),
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(5.)),
            ..Default::default()
        },
        children![
            Text::new("A:"),
            (
                PlayerAction::Skill1,
                SkillBookLocation,
                ItemLocationAccept::<SkillBook>::new()
            ),
            Text::new("Z:"),
            (
                PlayerAction::Skill2,
                SkillBookLocation,
                ItemLocationAccept::<SkillBook>::new()
            ),
            Text::new("E:"),
            (
                PlayerAction::Skill3,
                SkillBookLocation,
                ItemLocationAccept::<SkillBook>::new()
            ),
            Text::new("R:"),
            (
                PlayerAction::Skill4,
                SkillBookLocation,
                ItemLocationAccept::<SkillBook>::new()
            )
        ],
    )
}

fn create_panel(_trigger: Trigger<OnAdd, SkillsPanel>, mut commands: Commands) {
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
        if books.contains(item_entity) {
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
