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
pub struct SpawningMonster;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Life(u16);

impl Life {
    pub fn new(life: u16) -> Self {
        Life(life)
    }

    pub fn hit(&mut self, damage: u16) {
        if damage > self.0 {
            self.0 = 0;
        } else {
            self.0 -= damage;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.0 == 0
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}
