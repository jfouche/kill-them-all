use super::{
    dnd::{DndCursor, DraggedEntity},
    item_location::{ItemLocationDragObservers, ShowBorderOnDrag},
    popup_info::SpawnInfoPopupObservers,
};
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
    let mut observers = vec![Observer::new(on_drop_item)]
        .with_observers(SpawnInfoPopupObservers::observers())
        .with_observers(ShowBorderOnDrag::<With<SkillBook>>::observers())
        .with_observers(ItemLocationDragObservers::observers());

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
    skill_gems: Query<(), With<SkillBook>>,
) {
    if let Some(item_entity) = ***cursor {
        if skill_gems.get(item_entity).is_ok() {
            if let Ok(action) = locations.get(trigger.target()) {
                // The item dropped is a skill gem
                commands.trigger(EquipSkillBookEvent {
                    action: *action,
                    book_entity: item_entity,
                });
            }
        }
    }
}

// fn show_skills(
//     trigger: Trigger<OnAdd, SkillsPanel>,
//     mut commands: Commands,
//     player: Single<Entity, With<Player>>,
//     fireballs: Query<&Parent, With<FireBallLauncher>>,
//     shurikens: Query<&Parent, With<ShurikenLauncher>>,
//     mines: Query<&Parent, With<MineDropper>>,
//     death_auras: Query<&Parent, With<DeathAura>>,
//     assets: Res<SkillAssets>,
// ) {
//     let player = *player;
//     let mut panel_commands = commands.entity(trigger.entity());
//     fireballs
//         .iter()
//         .filter(|&parent| **parent == player)
//         .for_each(|_| {
//             spawn_skill::<FireBallLauncher>(&mut panel_commands, &assets);
//         });
//     shurikens
//         .iter()
//         .filter(|&parent| **parent == player)
//         .for_each(|_| {
//             spawn_skill::<ShurikenLauncher>(&mut panel_commands, &assets);
//         });
//     mines
//         .iter()
//         .filter(|&parent| **parent == player)
//         .for_each(|_| {
//             spawn_skill::<MineDropper>(&mut panel_commands, &assets);
//         });
//     death_auras
//         .iter()
//         .filter(|&parent| **parent == player)
//         .for_each(|_| {
//             spawn_skill::<DeathAura>(&mut panel_commands, &assets);
//         });
// }

// fn spawn_skill<T>(panel: &mut EntityCommands, assets: &SkillAssets)
// where
//     T: Component + SkillUI,
// {
//     let _text = [T::title(), T::label()].join("\n");
//     panel.with_child((
//         assets.image_node(T::tile_index()),
//         // ShowPopupOnMouseOver {
//         //     text,
//         //     image: Some(assets.image_node(T::tile_index())),
//         // },
//     ));
// }
