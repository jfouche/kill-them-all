use super::CharacteristicsPanel;
use super::InventoryPanel;
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
#[require(
    Popup(|| Popup::default().with_title("Pause")),
    Name(|| Name::new("PauseMenu"))
)]
struct PauseMenu;

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn(PauseMenu).with_children(|popup| {
        popup
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(95.),
                ..Default::default()
            })
            .with_children(|flex| {
                flex.spawn(InventoryPanel);
                flex.spawn(CharacteristicsPanel);
            });
    });
}
