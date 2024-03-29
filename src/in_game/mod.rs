mod bonus;
mod bullets;
mod collisions;
mod monster;
mod player;
mod round;
mod world;

use bevy::app::PluginGroupBuilder;

use crate::prelude::*;

pub use crate::in_game::collisions::*;

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
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(start_in_game))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(stop_in_game));
    }
}

fn start_in_game(mut conf: ResMut<RapierConfiguration>) {
    conf.physics_pipeline_active = true;
}

fn stop_in_game(mut conf: ResMut<RapierConfiguration>) {
    conf.physics_pipeline_active = false;
}
