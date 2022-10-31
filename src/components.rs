use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct Monster {
    pub speed: f32,
}

#[derive(Component)]
pub struct Bullet;