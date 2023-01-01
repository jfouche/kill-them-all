use std::time::Duration;

use bevy::prelude::*;

use super::Blink;

pub struct TemporaryPlugin;

impl Plugin for TemporaryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_timer).add_system(on_blink);
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Temporary {
    blink: Duration,
    timer: Timer,
    pause: bool,
}

impl Temporary {
    pub fn new(duration: Duration, blink: Duration) -> Self {
        assert!(
            duration > blink,
            "Can't create Temporary when duration < blink duration"
        );
        Temporary {
            blink,
            timer: Timer::new(duration - blink, TimerMode::Once),
            pause: false,
        }
    }

    pub fn pause(&mut self, pause: bool) {
        self.pause = pause;
    }
}

fn on_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Temporary), Without<Blink>>,
) {
    for (entity, mut temp) in query.iter_mut() {
        if !temp.pause {
            temp.timer.tick(time.delta());
            if temp.timer.just_finished() {
                // Blink
                let duration = temp.blink;
                temp.timer.set_duration(duration);
                temp.timer.reset();
                commands
                    .entity(entity)
                    .insert(Blink::new(Duration::from_millis(300)));
            }
        }
    }
}

fn on_blink(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Temporary), With<Blink>>,
) {
    for (entity, mut temp) in query.iter_mut() {
        if !temp.pause {
            temp.timer.tick(time.delta());
            if temp.timer.just_finished() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
