use crate::{components::*, schedule::*};
use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Helmet>()
            .register_type::<BodyArmour>()
            .add_systems(Update, regen_life.in_set(GameRunningSet::EntityUpdate));
    }
}

fn regen_life(mut query: Query<(&mut Life, &LifeRegen)>, time: Res<Time>) {
    for (mut life, regen) in &mut query {
        let life_per_sec = life.max_life() * (regen.increases / 100.);
        life.regenerate(life_per_sec * time.delta_seconds());
    }
}
