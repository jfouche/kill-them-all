use crate::components::*;
use crate::schedule::*;
use bevy::prelude::*;

pub struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Round>()
            .add_systems(Update, round_finish.in_set(GameRunningSet::EntityUpdate))
            .add_systems(OnEnter(InGameState::RoundEnd), despawn_all::<Monster>);
    }
}

fn round_finish(
    time: Res<Time>,
    mut round: ResMut<Round>,
    mut state: ResMut<NextState<InGameState>>,
) {
    round.timer.tick(time.delta());
    if round.timer.just_finished() {
        round.level += 1;
        state.set(InGameState::RoundEnd);
    }
}
