pub use crate::{components::*, events::*, resources::*, utils::*};
pub use bevy::prelude::*;
pub use bevy_rapier2d::prelude::*;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet, States)]
pub enum GameState {
    #[default]
    InGame,
    GamePaused,
    LevelUp,
}
