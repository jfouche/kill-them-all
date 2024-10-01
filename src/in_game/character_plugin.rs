use crate::{components::*, schedule::*};
use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MovementSpeed>()
            .register_type::<Life>()
            .register_type::<MaxLife>()
            .register_type::<BaseLife>()
            .register_type::<AttackSpeed>()
            .register_type::<PierceChance>()
            .register_type::<Equipment>()
            .register_type::<Helmet>()
            .register_type::<BodyArmour>()
            .register_type::<Boots>()
            .add_systems(PreUpdate, update_skills.run_if(game_is_running))
            .add_systems(Update, regen_life.in_set(GameRunningSet::EntityUpdate));
    }
}

fn update_skills(
    mut query: Query<(
        &mut Life,
        &mut MaxLife,
        &BaseLife,
        &Helmet,
        &BodyArmour,
        &Boots,
    )>,
) {
    for (mut life, mut max_life, base_life, helmet, body_armour, boots) in &mut query {
        let more_life = helmet.more_life() + body_armour.more_life() + boots.more_life();
        max_life.0 = **base_life + more_life;
        life.check(*max_life);
    }
}

fn regen_life(mut query: Query<(&mut Life, &MaxLife, &LifeRegen)>, time: Res<Time>) {
    for (mut life, max_life, regen) in &mut query {
        let life_per_sec = **max_life * (regen.increases / 100.);
        life.regenerate(life_per_sec * time.delta_seconds());
        life.check(*max_life);
    }
}
