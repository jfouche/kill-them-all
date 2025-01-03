mod life_bar_plugin;
mod map_level_plugin;
mod xp_bar_plugin;

use super::GameState;
use crate::components::despawn_all;
use bevy::prelude::*;

/// Component to add to HUD entities
#[derive(Component, Default)]
struct Hud;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            life_bar_plugin::LifeBarPlugin,
            xp_bar_plugin::ExperienceBarPlugin,
            map_level_plugin::MapLevelPlugin,
        ))
        .add_systems(OnExit(GameState::InGame), despawn_all::<Hud>);
    }
}
