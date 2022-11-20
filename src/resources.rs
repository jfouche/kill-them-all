use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct ScoreResource(pub u16);

#[derive(Resource, Deref)]
pub struct UiFont(pub Handle<Font>);

#[derive(Resource)]
pub struct PlayerFireConfig {
    /// timer between attacks per seconds
    pub timer: Timer,
}
