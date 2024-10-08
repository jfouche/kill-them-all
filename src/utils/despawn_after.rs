use super::blink::Blink;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct DespawnAfter {
    timer: Timer,
    pause: bool,
    blink: Option<Timer>,
}

impl DespawnAfter {
    pub fn new(duration: Duration) -> Self {
        DespawnAfter {
            timer: Timer::new(duration, TimerMode::Once),
            pause: false,
            blink: None,
        }
    }

    pub fn pause(&mut self, pause: bool) {
        self.pause = pause;
    }

    pub fn with_blink(mut self, duration: Duration) -> Self {
        let duration = self.timer.duration() - duration;
        self.blink = Some(Timer::new(duration, TimerMode::Once));
        self
    }
}

pub fn despawn_after_plugin(app: &mut App) {
    app.add_systems(Update, (start_blink, despawn_entity));
}

fn start_blink(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnAfter)>,
    time: Res<Time>,
) {
    for (entity, mut despawn_after) in &mut query {
        if !despawn_after.pause {
            if let Some(ref mut timer) = despawn_after.blink {
                timer.tick(time.delta());
                if timer.just_finished() {
                    commands
                        .entity(entity)
                        .insert(Blink::new(Duration::from_secs_f32(0.33)));
                }
            }
        }
    }
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
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
