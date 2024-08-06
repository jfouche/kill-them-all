mod bonus_plugin;
mod collisions_plugin;
mod monster_plugin;
mod player_plugin;
mod round_plugin;
mod world_plugin;

use crate::prelude::*;
use bevy::app::PluginGroupBuilder;

pub struct InGamePluginsGroup;

impl PluginGroup for InGamePluginsGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InGamePlugin)
            .add(bonus_plugin::BonusPlugin)
            .add(collisions_plugin::CollisionsPlugin)
            .add(monster_plugin::MonsterPlugin)
            .add(player_plugin::PlayerPlugin)
            .add(round_plugin::RoundPlugin)
            .add(world_plugin::WorldPlugin)
    }
}

struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Round::new(15.))
            .add_systems(OnEnter(GameState::InGame), start_in_game)
            .add_systems(OnExit(GameState::InGame), stop_in_game);
    }
}

fn start_in_game(mut conf: ResMut<RapierConfiguration>) {
    conf.physics_pipeline_active = true;
}

fn stop_in_game(mut conf: ResMut<RapierConfiguration>) {
    conf.physics_pipeline_active = false;
}
