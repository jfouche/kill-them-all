use crate::{components::*, schedule::*};
use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Equipment>()
            .register_type::<Helmet>()
            .register_type::<BodyArmour>()
            .register_type::<Boots>()
            .add_systems(PreUpdate, update_life)
            .add_systems(Update, regen_life.in_set(GameRunningSet::EntityUpdate));
    }
}

fn update_life(mut query: Query<(&mut Life, &Helmet, &BodyArmour, &Boots)>) {
    for (mut life, helmet, body_armour, boots) in &mut query {
        let mut more_life = helmet.more_life() + body_armour.more_life() + boots.more_life();
    }
}

fn regen_life(mut query: Query<(&mut Life, &LifeRegen)>, time: Res<Time>) {
    for (mut life, regen) in &mut query {
        let life_per_sec = life.max_life() * (regen.increases / 100.);
        life.regenerate(life_per_sec * time.delta_seconds());
    }
}
