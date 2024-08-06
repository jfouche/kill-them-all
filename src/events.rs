use bevy::prelude::*;

/// Event to notify a monster was hit
#[derive(Event)]
pub struct MonsterHitEvent {
    pub entity: Entity,
    pub damage: u16,
}

impl MonsterHitEvent {
    pub fn new(entity: Entity, damage: u16) -> Self {
        MonsterHitEvent { entity, damage }
    }
}

// Event to notify a monster died
#[derive(Event)]
pub struct MonsterDeathEvent {
    pub entity: Entity,
    pub pos: Vec3,
}

// Event to notify the player was hit
#[derive(Event)]
pub struct PlayerHitEvent {
    pub entity: Entity,
}

impl PlayerHitEvent {
    pub fn new(entity: Entity) -> Self {
        PlayerHitEvent { entity }
    }
}

// Event to notify the player died
#[derive(Event)]
pub struct PlayerDeathEvent;

// Event to notify a player level up
#[derive(Event)]
pub struct LevelUpEvent;
