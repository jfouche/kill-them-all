use super::{CharacteristicsPanel, EquipmentsPanel, InventoryPanel, SkillsPanel};
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
    Name(|| Name::new("PauseMenu")),
    Popup(|| Popup::default().with_title("Pause"))
)]
struct PauseMenu;

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn(PauseMenu).with_children(|menu| {
        menu.spawn(VSizer).with_children(|vsizer| {
            vsizer.spawn(HSizer).with_children(|hsizer| {
                hsizer.spawn(EquipmentsPanel);
                hsizer.spawn(InventoryPanel);
            });
            vsizer.spawn(SkillsPanel);
            vsizer.spawn(CharacteristicsPanel);
        });
    });
}
