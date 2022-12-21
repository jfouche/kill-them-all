use std::time::Duration;

use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct ScoreResource(pub u16);

#[derive(Resource, Deref)]
pub struct UiFont(pub Handle<Font>);

#[derive(Resource, Default)]
pub struct GameTextures {
    pub money: Handle<Image>,
    pub monster: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct PlayerConfig {
    pub life: u16,
    pub movement_speed: f32,
    pub attack_speed: f32,
}

#[derive(Resource)]
pub struct Round {
    level: u16,
    timer: Timer,
}

impl Round {
    /// Initialise round of `duration`seconds
    pub fn new(duration: f32) -> Self {
        Round {
            level: 0,
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
        }
    }

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
