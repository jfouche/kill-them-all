use crate::{components::*, schedule::InGameSet};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct BonusPlugin;

impl Plugin for BonusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_asset)
            .add_systems(Update, spawn_bonus.in_set(InGameSet::EntityUpdate));
    }
}

const MONEY_ASSET_PATH: &str = "items/crystal_01a.png";

fn load_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = BonusAssets {
        texture: asset_server.load(MONEY_ASSET_PATH),
    };
    commands.insert_resource(assets);
}

fn spawn_bonus(
    mut commands: Commands,
    mut monster_death_events: EventReader<MonsterDeathEvent>,
    assets: Res<BonusAssets>,
) {
    let mut rng = thread_rng();
    for event in monster_death_events.read() {
        if rng.gen_range(0..100) < 20 {
            commands.spawn(BonusBundle::new(event.pos, &assets));
        }
    }
}
