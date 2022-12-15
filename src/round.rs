use crate::prelude::*;

pub struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_round_timer).add_system_set(
            SystemSet::on_update(GameState::InGame).with_system(round_duration_timer),
        );
    }
}

fn init_round_timer(mut commands: Commands) {
    commands.spawn(RoundTimer::new());
}

fn round_duration_timer(
    mut query: Query<&mut RoundTimer>,
    time: Res<Time>,
    mut round: ResMut<Round>,
) {
    if let Ok(mut timer) = query.get_single_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            round.0 += 1;
        }
    }
}
