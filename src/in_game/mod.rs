mod bonus;
mod bullets;
mod collisions;
mod monster;
mod player;
mod round;
mod world;

use bevy::app::PluginGroupBuilder;

use crate::prelude::*;

pub struct InGamePluginsGroup;

impl PluginGroup for InGamePluginsGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InGamePlugin)
            .add(bonus::BonusPlugin)
            .add(collisions::CollisionsPlugin)
            .add(monster::MonsterPlugin)
            .add(player::PlayerPlugin)
            .add(round::RoundPlugin)
            .add(world::WorldPlugin)
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
