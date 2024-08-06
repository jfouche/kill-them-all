use crate::prelude::*;

pub struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            round_duration_timer.run_if(in_state(GameState::InGame)),
        );
    }
}

fn round_duration_timer(time: Res<Time>, mut round: ResMut<Round>) {
    round.tick(time.delta());
}
