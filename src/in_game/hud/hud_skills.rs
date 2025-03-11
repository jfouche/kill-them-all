use super::Hud;
use crate::{
    components::{
        inventory::{PlayerEquipmentChanged, ToggleInventory},
        player::PlayerAction,
        skills::SkillGemLocation,
    },
    in_game::menu::popup_info::SpawnInfoPopupObservers,
    schedule::GameState,
    ui::button::TextButton,
    utils::observers::VecObserversExt,
};
use bevy::prelude::*;

pub struct HudSkillsPlugin;

impl Plugin for HudSkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_hud_skills);
    }
}

#[derive(Component)]
#[require(
    Hud,
    Name(|| Name::new("HudSkills")),
    Node(|| Node {
        position_type: PositionType::Absolute,
        right: Val::Px(50.),
        bottom: Val::Px(15.),
        height: Val::Px(32.),
        ..Default::default()
    })
)]
struct HudSkillsPanel;

fn spawn_hud_skills(mut commands: Commands) {
    let mut observers = vec![].with_observers(SpawnInfoPopupObservers::observers());
    commands.spawn(HudSkillsPanel).with_children(|p| {
        p.spawn(TextButton::small("I")).observe(
            |_: Trigger<Pointer<Click>>, mut command: Commands| {
                command.trigger(ToggleInventory);
            },
        );

        let entity = p.spawn((PlayerAction::Skill1, SkillGemLocation)).id();
        observers.watch_entity(entity);
        let entity = p.spawn((PlayerAction::Skill2, SkillGemLocation)).id();
        observers.watch_entity(entity);
        let entity = p.spawn((PlayerAction::Skill3, SkillGemLocation)).id();
        observers.watch_entity(entity);
        let entity = p.spawn((PlayerAction::Skill4, SkillGemLocation)).id();
        observers.watch_entity(entity);
    });
    commands.spawn_batch(observers);

    // to force to init the update
    commands.trigger(PlayerEquipmentChanged);
}
