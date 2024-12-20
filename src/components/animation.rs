use bevy::prelude::*;

///
/// The animation [AnimationTimer] component indicates that the entity should
/// be animated
///
#[derive(Component, Deref, DerefMut, Reflect)]
pub struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

/// An [AnimationTimer] which cyclicly update the index of a [Sprite]
/// with [TextureAtlas]
#[derive(Component, Default)]
#[require(AnimationTimer, Sprite)]
pub struct CyclicAnimation {
    cycle: Vec<usize>,
    current: usize
}

impl CyclicAnimation {
    pub fn new(cycle: &[usize]) -> Self {
        CyclicAnimation { cycle: cycle.into(), current: 0 }
    }

    /// Returns the current index, and move to the next one
    pub fn next(&mut self) -> usize {
        let current = self.cycle.get(self.current).unwrap_or(&0);
        self.current = (self.current + 1) % self.cycle.len();
        *current
    }
}

/// A one shot animation
#[derive(Component)]
#[require(AnimationTimer, Sprite)]
pub struct OneShotAnimation {
    indexes: Vec<usize>,
    current: usize
}

impl OneShotAnimation {
    pub fn new(indexes: &[usize]) -> Self {
        OneShotAnimation { indexes: indexes.into(), current: 0 }
    }

    /// Returns the current index, and move to the next one.
    /// 
    /// Return None at the end
    pub fn next(&mut self) -> Option<usize> {
        let current = self.indexes.get(self.current)?;
        self.current += 1;
        Some(*current)
    }

    /// Returns true if the animation is finished
    pub fn finished(&self) -> bool {
        self.current >= self.indexes.len()
    }
}