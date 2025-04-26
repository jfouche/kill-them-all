use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct DespawnAfter {
    timer: Timer,
    pause: bool,
}

impl DespawnAfter {
    pub fn new(duration: Duration) -> Self {
        DespawnAfter {
            timer: Timer::new(duration, TimerMode::Once),
            pause: false,
        }
    }

    pub fn pause(&mut self, pause: bool) {
        self.pause = pause;
    }
}

pub fn despawn_after_plugin(app: &mut App) {
    app.add_systems(Update, despawn_entity);
}

fn despawn_entity(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnAfter)>,
    time: Res<Time>,
) {
    for (entity, mut despawn_after) in &mut query {
        if !despawn_after.pause {
            despawn_after.timer.tick(time.delta());
            if despawn_after.timer.just_finished() {
                commands.entity(entity).despawn();
            }
        }
    }
}
