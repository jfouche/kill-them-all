use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct ScoreResource(pub u16);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
}
