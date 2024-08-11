use crate::components::Round;
use crate::schedule::*;
use bevy::prelude::*;

pub struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Round>().add_systems(
            Update,
            round_duration_timer.in_set(GameRunningSet::EntityUpdate),
        );
    }
}

fn round_duration_timer(time: Res<Time>, mut round: ResMut<Round>) {
    round.tick(time.delta());
}
