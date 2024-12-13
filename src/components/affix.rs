use bevy::prelude::*;

#[derive(Component, Default, Copy, Clone)]
pub struct Affix;

#[derive(Component, Default, Deref, Debug, Reflect)]
pub struct MoreLife(pub f32);

impl std::fmt::Display for MoreLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0} to maximum life", self.0)
    }
}

#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct IncreaseMaxLife(pub f32);

impl std::fmt::Display for IncreaseMaxLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Increase {:.0}% maximum life", self.0)
    }
}

#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct MoreArmour(pub f32);

#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct IncreaseMovementSpeed(pub f32);

impl std::fmt::Display for IncreaseMovementSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{:.0}% movement speed", self.0)
    }
}
