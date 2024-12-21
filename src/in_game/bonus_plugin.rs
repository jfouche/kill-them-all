use super::GameState;
use crate::{components::*, schedule::GameRunningSet};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct BonusPlugin;

impl Plugin for BonusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BonusAssets>()
            .add_systems(OnExit(GameState::InGame), despawn_all::<Bonus>)
            .add_systems(Update, spawn_bonus.in_set(GameRunningSet::EntityUpdate));
    }
}

fn spawn_bonus(
    mut commands: Commands,
    mut monster_death_events: EventReader<MonsterDeathEvent>,
    assets: Res<BonusAssets>,
) {
    let mut rng = thread_rng();
    for event in monster_death_events.read() {
        if rng.gen_range(0..100) < 20 {
            commands.spawn((
                Bonus,
                Bonus::sprite(&assets),
                Transform::from_translation(event.pos),
            ));
        }
    }
}
