use super::GameState;
use crate::{components::*, schedule::GameRunningSet};
use bevy::prelude::*;
use rand::thread_rng;

pub struct BonusPlugin;

impl Plugin for BonusPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bonus>()
            .add_systems(OnExit(GameState::InGame), despawn_all::<Bonus>)
            .add_systems(Update, spawn_bonus.in_set(GameRunningSet::EntityUpdate));
    }
}

fn spawn_bonus(
    mut commands: Commands,
    mut monster_death_events: EventReader<MonsterDeathEvent>,
    assets: Res<EquipmentAssets>,
) {
    let mut rng = thread_rng();
    for event in monster_death_events.read() {
        if let Some(equipment_info) = BonusProvider::spawn(&mut commands, &mut rng) {
            commands.spawn((
                Bonus(equipment_info.entity),
                assets.sprite(equipment_info.info.tile_index),
                Transform::from_translation(event.pos).with_scale(Vec3::splat(0.3)),
            ));
        }
    }
}
