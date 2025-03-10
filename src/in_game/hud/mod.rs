mod hud_skills;
mod life_bar_plugin;
mod map_level_plugin;
mod xp_bar_plugin;

pub use plugin::{Hud, HudPlugin};

mod plugin {
    use super::{
        hud_skills::HudSkillsPlugin, life_bar_plugin::LifeBarPlugin,
        map_level_plugin::MapLevelPlugin, xp_bar_plugin::ExperienceBarPlugin,
    };
    use crate::{components::despawn_all, schedule::GameState};
    use bevy::prelude::*;

    /// Component to add to HUD entities
    #[derive(Component, Default)]
    pub struct Hud;

    pub struct HudPlugin;

    impl Plugin for HudPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((
                LifeBarPlugin,
                ExperienceBarPlugin,
                MapLevelPlugin,
                HudSkillsPlugin,
            ))
            .add_systems(OnExit(GameState::InGame), despawn_all::<Hud>);
        }
    }
}
