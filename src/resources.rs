use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct ScoreResource(pub u16);

#[derive(Resource, Deref)]
pub struct UiFont(pub Handle<Font>);

#[derive(Resource, Default)]
pub struct GameTextures {
    pub money: Handle<Image>,
}

#[derive(Resource)]
pub struct PlayerConfig {
    pub life: u16,
    pub movement_speed: f32,
    pub attack_speed: f32,
}

#[derive(Resource)]
pub struct Round(pub u16);