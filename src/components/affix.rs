use super::Label;
use bevy::prelude::*;

#[derive(Component, Default, Copy, Clone)]
pub struct Affix;

#[derive(Component, Default, Deref, Debug, Reflect)]
pub struct MoreLife(pub f32);

impl Label for MoreLife {
    fn label(&self) -> String {
        format!("{:.0} to maximum life", self.0)
    }
}

#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct IncreaseMaxLife(pub f32);

impl Label for IncreaseMaxLife {
    fn label(&self) -> String {
        format!("Increase {:.0}% maximum life", self.0)
    }
}

#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct MoreArmour(pub f32);

#[derive(Component, Default, Clone, Copy, Deref, DerefMut, Debug, Reflect)]
pub struct IncreaseMovementSpeed(pub f32);

impl Label for IncreaseMovementSpeed {
    fn label(&self) -> String {
        format!("+{:.0}% movement speed", self.0)
    }
}
