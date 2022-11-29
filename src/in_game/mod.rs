mod bonus;
mod bullets;
mod collisions;
mod monster;
mod player;
mod world;

use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

pub struct InGamePlugins;

impl PluginGroup for InGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(bonus::BonusPlugin)
            .add(collisions::CollisionsPlugin)
            .add(monster::MonsterPlugin)
            .add(player::PlayerPlugin)
            .add(world::WorldPlugin)
    }
}
