use bevy::prelude::*;

/// Event to notify a monster was hit
pub struct MonsterHitEvent {
    pub entity: Entity,
}

impl MonsterHitEvent {
    pub fn new(entity: Entity) -> Self {
        MonsterHitEvent { entity }
    }
}

pub struct PlayerHitEvent {
    pub entity: Entity,
}

impl PlayerHitEvent {
    pub fn new(entity: Entity) -> Self {
        PlayerHitEvent { entity }
    }
}
