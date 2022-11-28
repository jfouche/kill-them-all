pub use crate::{components::*, events::*, resources::*};
pub use bevy::prelude::*;
pub use bevy_rapier2d::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    InGame,
    GamePaused,
}
