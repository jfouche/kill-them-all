use crate::{components::*, schedule::*};
use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MovementSpeed>()
            .register_type::<Life>()
            .register_type::<MaxLife>()
            .register_type::<BaseLife>()
            .register_type::<MovementSpeed>()
            .register_type::<BaseMovementSpeed>()
            .register_type::<IncreaseAttackSpeed>()
            .register_type::<PierceChance>()
            .register_type::<Armour>()
            .register_type::<Upgrades>()
            .register_type::<Equipments>()
            .register_type::<Equipment>()
            .register_type::<Helmet>()
            .register_type::<BodyArmour>()
            .register_type::<Boots>()
            .add_systems(
                PreUpdate,
                (
                    update_armour,
                    update_life,
                    update_life_regen,
                    update_movement_speed,
                    update_attack_speed,
                    update_pierce_chance,
                )
                    .run_if(game_is_running),
            )
            .add_systems(Update, regen_life.in_set(GameRunningSet::EntityUpdate));
    }
}

fn update_armour(mut query: Query<(&mut Armour, &Upgrades, &Equipments)>) {
    for (mut armour, upgrades, equipments) in &mut query {
        armour.0 = equipments.armour() + upgrades.armour();
    }
}

fn update_life(mut query: Query<((&mut Life, &mut MaxLife, &BaseLife), &Upgrades, &Equipments)>) {
    for ((mut life, mut max_life, base_life), upgrades, equipments) in &mut query {
        let more_life = equipments.more_life() + upgrades.more_life();
        let inc_life = equipments.increase_max_life() + upgrades.increase_max_life();
        max_life.0 = (**base_life + more_life) * (1. + inc_life / 100.);
        life.check(*max_life);
    }
}

fn update_life_regen(mut query: Query<(&mut LifeRegen, &Upgrades, &Equipments)>) {
    for (mut life_regen, upgrades, equipments) in &mut query {
        life_regen.0 = equipments.life_regen() + upgrades.life_regen();
    }
}

fn update_movement_speed(
    mut query: Query<(
        (&mut MovementSpeed, &BaseMovementSpeed),
        &Upgrades,
        &Equipments,
    )>,
) {
    for ((mut movement_speed, base_movement_speed), upgrades, equipments) in &mut query {
        let inc_move_speed =
            equipments.increase_movement_speed() + upgrades.increase_movement_speed();
        movement_speed.0 = **base_movement_speed * (1. + inc_move_speed / 100.);
    }
}

fn update_attack_speed(mut query: Query<(&mut IncreaseAttackSpeed, &Upgrades, &Equipments)>) {
    for (mut attack_speed, upgrades, equipments) in &mut query {
        // Attack speed
        attack_speed.0 = equipments.increase_attack_speed() + upgrades.increase_attack_speed();
    }
}

fn update_pierce_chance(mut query: Query<(&mut PierceChance, &Upgrades, &Equipments)>) {
    for (mut pierce_chance, upgrades, equipments) in &mut query {
        // Pierce chance
        pierce_chance.0 = equipments.pierce_chance() + upgrades.pierce_chance();
    }
}

fn regen_life(mut query: Query<(&mut Life, &MaxLife, &LifeRegen)>, time: Res<Time>) {
    for (mut life, max_life, regen) in &mut query {
        let life_per_sec = **max_life * (**regen / 100.);
        life.regenerate(life_per_sec * time.delta_seconds());
        life.check(*max_life);
    }
}
