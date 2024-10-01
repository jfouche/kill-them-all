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

fn update_life(mut query: Query<(&mut Life, &BaseLife, &Helmet, &BodyArmour, &Boots)>) {
    for (mut life, base_life, helmet, body_armour, boots) in &mut query {
        let more_life = helmet.more_life() + body_armour.more_life() + boots.more_life();
        let max_life = **base_life + more_life;
        life.set_max(max_life);
    }
}

fn regen_life(mut query: Query<(&mut Life, &LifeRegen)>, time: Res<Time>) {
    for (mut life, regen) in &mut query {
        let life_per_sec = life.max_life() * (regen.increases / 100.);
        life.regenerate(life_per_sec * time.delta_seconds());
    }
}
