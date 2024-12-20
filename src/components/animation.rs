use bevy::prelude::*;
use std::ops::Range;

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

/// Animation indexes
#[derive(Deref)]
pub struct AnimationIndexes(Vec<usize>);

impl<const SIZE: usize> From<&[usize; SIZE]> for AnimationIndexes {
    fn from(value: &[usize; SIZE]) -> Self {
        AnimationIndexes(value.into())
    }
}

impl From<Range<usize>> for AnimationIndexes {
    fn from(value: Range<usize>) -> Self {
        AnimationIndexes(Vec::from_iter(value))
    }
}

/// An [AnimationTimer] which cyclicly update the index of a [Sprite]
/// with [TextureAtlas]
#[derive(Component)]
#[require(AnimationTimer, Sprite)]
pub struct CyclicAnimation {
    indexes: AnimationIndexes,
    current: usize,
}

impl CyclicAnimation {
    pub fn new(indexes: impl Into<AnimationIndexes>) -> Self {
        CyclicAnimation {
            indexes: indexes.into(),
            current: 0,
        }
    }

    /// Returns the current index, and move to the next one
    pub fn next(&mut self) -> usize {
        let current = self.indexes.get(self.current).unwrap_or(&0);
        self.current = (self.current + 1) % self.indexes.len();
        *current
    }
}

/// A one shot animation
#[derive(Component)]
#[require(AnimationTimer, Sprite)]
pub struct OneShotAnimation {
    indexes: AnimationIndexes,
    current: usize,
}

impl OneShotAnimation {
    pub fn new(indexes: impl Into<AnimationIndexes>) -> Self {
        OneShotAnimation {
            indexes: indexes.into(),
            current: 0,
        }
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
