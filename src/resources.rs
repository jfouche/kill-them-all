use std::time::Duration;

use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct ScoreResource(pub u16);

#[derive(Resource, Deref)]
pub struct UiFont(pub Handle<Font>);

#[derive(Resource)]
pub struct Round {
    level: u16,
    timer: Timer,
}

impl Default for Round {
    fn default() -> Self {
        Round {
            level: 0,
            timer: Timer::from_seconds(10., TimerMode::Repeating),
        }
    }
}

impl Round {
    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.just_finished() {
            self.level += 1;
        }
    }

    pub fn level(&self) -> u16 {
        self.level
    }
}
