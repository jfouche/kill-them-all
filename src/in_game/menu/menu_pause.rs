use super::characteristics_panel;
use super::inventory_panel;
use crate::components::*;
use crate::in_game::back_to_game;
use crate::schedule::*;
use crate::ui::*;
use bevy::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::Pause), spawn_pause_menu)
            .add_systems(OnExit(InGameState::Pause), despawn_all::<PauseMenu>)
            .add_systems(Update, back_to_game.run_if(in_state(InGameState::Pause)));
    }
}

#[derive(Component)]
struct PauseMenu;

fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn_popup("Pause", (PauseMenu, Name::new("PauseMenu")))
        .with_children(|popup| {
            popup
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(95.),
                    ..Default::default()
                })
                .with_children(|flex| {
                    flex.spawn(inventory_panel());
                    flex.spawn(characteristics_panel());
                });
        });
}
