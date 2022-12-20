use crate::prelude::*;

pub struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame).with_system(round_duration_timer),
        );
    }
}

fn round_duration_timer(time: Res<Time>, mut round: ResMut<Round>) {
    round.tick(time.delta());
}
