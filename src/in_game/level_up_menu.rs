use crate::components::*;
use crate::schedule::*;
use crate::ui::{spawn_button, spawn_popup};
use bevy::prelude::*;

use super::back_to_game;

pub struct LevelUpMenuPlugin;

impl Plugin for LevelUpMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enter_level_up_state.in_set(InGameSet::EntityUpdate))
            .add_systems(OnEnter(InGameState::LevelUp), spawn_level_up_menu)
            .add_systems(OnExit(InGameState::LevelUp), despawn_all::<LevelUpMenu>)
            .add_systems(
                Update,
                (
                    back_to_game,
                    upgrade_skill::<MaxLifeButton>,
                    upgrade_skill::<MovementSpeedButton>,
                    upgrade_skill::<AttackSpeedButton>,

                )
                    .run_if(in_state(InGameState::LevelUp)),
            )
            // .add_systems(Update, (button_system, print_nav_events))
            ;
    }
}

fn enter_level_up_state(
    mut level_up_rcv: EventReader<LevelUpEvent>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if level_up_rcv.read().next().is_some() {
        warn!("enter_level_up_state");
        next_state.set(InGameState::LevelUp);
    }
}

#[derive(Component)]
struct LevelUpMenu;

///
/// Trait to print a player skill
///
trait UpgradeSkill {
    /// Component of the player skill
    type SkillComponent: Component;

    /// upgrade the skill
    fn upgrade(component: &mut Self::SkillComponent);
}

#[derive(Component)]
struct MaxLifeButton;

impl UpgradeSkill for MaxLifeButton {
    type SkillComponent = Life;
    fn upgrade(component: &mut Self::SkillComponent) {
        component.increases(10.);
    }
}

#[derive(Component)]
struct AttackSpeedButton;

impl UpgradeSkill for AttackSpeedButton {
    type SkillComponent = AttackSpeed;
    fn upgrade(component: &mut Self::SkillComponent) {
        component.increases(10.);
    }
}

#[derive(Component)]
struct MovementSpeedButton;

impl UpgradeSkill for MovementSpeedButton {
    type SkillComponent = MovementSpeed;
    fn upgrade(component: &mut Self::SkillComponent) {
        component.increases(10.);
    }
}

fn spawn_level_up_menu(commands: Commands) {
    spawn_popup(commands, "Level up!", LevelUpMenu, |window| {
        spawn_button(window, "Max life", MaxLifeButton);
        spawn_button(window, "Attack speed", AttackSpeedButton);
        spawn_button(window, "Movement speed", MovementSpeedButton);
    });
}

///
/// Upgrade a skill of the player, returning back to game
///
fn upgrade_skill<T: UpgradeSkill + Component>(
    mut q_btn: Query<&Interaction, (Changed<Interaction>, With<T>)>,
    mut q_player: Query<&mut T::SkillComponent, With<Player>>,
    mut state: ResMut<NextState<InGameState>>,
) {
    if let Ok(mut skill) = q_player.get_single_mut() {
        for interaction in &mut q_btn {
            if *interaction == Interaction::Pressed {
                T::upgrade(&mut skill);
                state.set(InGameState::Running);
            }
        }
    }
}
