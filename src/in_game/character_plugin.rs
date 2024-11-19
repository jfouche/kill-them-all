use crate::{components::*, schedule::*};
use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HitEvent>()
            .register_type::<BaseLife>()
            .register_type::<Life>()
            .register_type::<MaxLife>()
            .register_type::<MoreLife>()
            .register_type::<IncreaseMaxLife>()
            .register_type::<LifeRegen>()
            .register_type::<BaseMovementSpeed>()
            .register_type::<MovementSpeed>()
            .register_type::<IncreaseMovementSpeed>()
            .register_type::<IncreaseAttackSpeed>()
            .register_type::<PierceChance>()
            .register_type::<Armour>()
            .register_type::<MoreArmour>()
            // .register_type::<Helmet>()
            // .register_type::<BodyArmour>()
            // .register_type::<Boots>()
            .add_systems(
                PreUpdate,
                (
                    update_armour,
                    update_life,
                    update_life_regen,
                    update_movement_speed,
                    // update_attack_speed,
                    // update_pierce_chance,
                )
                    .run_if(game_is_running),
            )
            .add_systems(Update, regen_life.in_set(GameRunningSet::EntityUpdate));
    }
}

// fn reset_affix<T>(mut characters: Query<&mut T, With<Character>>)
// where
//     T: Component + Default,
// {
//     for mut affix in &mut characters {
//         *affix = T::default();
//     }
// }

// fn update_more_affix<A, T>(
//     affixes: Query<(&A, &Parent)>,
//     mut characters: Query<&mut T, With<Character>>,
// ) where
//     A: Component + Deref,
//     T: Component + DerefMut,
//     <A as Deref>::Target: Copy,
//     <T as Deref>::Target: std::ops::AddAssign<<A as Deref>::Target>,
// {
//     for (more_affix, parent) in &affixes {
//         if let Ok(mut comp) = characters.get_mut(**parent) {
//             **comp += **more_affix;
//         }
//     }
// }

/// [Armour] = sum([Armour]) + sum ([MoreArmour])
fn update_armour(
    mut characters: Query<&mut Armour, With<Character>>,
    armours: Query<(&Armour, &Parent), Without<Character>>,
    more_armours: Query<(&MoreArmour, &Parent), Without<Character>>,
) {
    for mut char_armour in &mut characters {
        **char_armour = 0.;
    }
    for (armour, parent) in &armours {
        if let Ok(mut char_armour) = characters.get_mut(**parent) {
            **char_armour += **armour;
        }
    }
    for (more_armour, parent) in &more_armours {
        if let Ok(mut char_armour) = characters.get_mut(**parent) {
            **char_armour += **more_armour;
        }
    }
}

/// [MaxLife] = ([BaseLife] + sum([MoreLife])) * sum([IncreaseLife]) %
fn update_life(
    mut characters: Query<(&BaseLife, &mut MaxLife, &mut IncreaseMaxLife), With<Character>>,
    more_affixes: Query<(&MoreLife, &Parent), Without<Character>>,
    incr_affixes: Query<(&IncreaseMaxLife, &Parent), Without<Character>>,
) {
    for (base_life, mut max_life, mut incr_life) in &mut characters {
        **max_life = **base_life;
        **incr_life = 0.;
    }
    for (more_life, parent) in &more_affixes {
        if let Ok((_base_life, mut max_life, _incr_life)) = characters.get_mut(**parent) {
            **max_life += **more_life;
        }
    }
    for (incr_life, parent) in &incr_affixes {
        if let Ok((_base_life, _life, mut incr_char_life)) = characters.get_mut(**parent) {
            **incr_char_life += **incr_life;
        }
    }

    for (_base_life, mut max_life, incr_life) in &mut characters {
        **max_life *= 1. + **incr_life / 100.;
    }
}

/// [LifeRegen] = sum([LifeRegen])
fn update_life_regen(
    mut characters: Query<&mut LifeRegen, With<Character>>,
    affixes: Query<(&LifeRegen, &Parent), Without<Character>>,
) {
    for mut char_life_regen in &mut characters {
        **char_life_regen = 0.;
    }
    for (life_regen, parent) in &affixes {
        if let Ok(mut char_life_regen) = characters.get_mut(**parent) {
            **char_life_regen += **life_regen;
        }
    }
}

/// [MovementSpeed] = [BaseMovementSpeed] * sum([IncreaseMovementSpeed]) %
fn update_movement_speed(
    mut characters: Query<
        (
            &BaseMovementSpeed,
            &mut MovementSpeed,
            &mut IncreaseMovementSpeed,
        ),
        With<Character>,
    >,
    affixes: Query<(&IncreaseMovementSpeed, &Parent), Without<Character>>,
) {
    for (_, mut move_speed, mut incr_move_speed) in &mut characters {
        **move_speed = 0.;
        **incr_move_speed = 0.;
    }
    for (incr_move_speed, parent) in &affixes {
        if let Ok((_, _, mut char_incr_move_speed)) = characters.get_mut(**parent) {
            **char_incr_move_speed += **incr_move_speed;
        }
    }
    for (base_move_speed, mut move_speed, incr_move_speed) in &mut characters {
        **move_speed = **base_move_speed * (1. + **incr_move_speed / 100.);
    }
}

// fn update_attack_speed(mut query: Query<(&mut IncreaseAttackSpeed, &Upgrades, &Equipments)>) {
//     for (mut attack_speed, upgrades, equipments) in &mut query {
//         // Attack speed
//         attack_speed.0 = equipments.increase_attack_speed() + upgrades.increase_attack_speed();
//     }
// }

// /// [PierceChance] = sum([PierceChance])
// fn update_pierce_chance(mut query: Query<(&mut PierceChance, &Upgrades, &Equipments)>) {
//     for (mut pierce_chance, upgrades, equipments) in &mut query {
//         // Pierce chance
//         pierce_chance.0 = equipments.pierce_chance() + upgrades.pierce_chance();
//     }
// }

fn regen_life(mut query: Query<(&mut Life, &MaxLife, &LifeRegen)>, time: Res<Time>) {
    for (mut life, max_life, regen) in &mut query {
        let life_per_sec = **max_life * (**regen / 100.);
        life.regenerate(life_per_sec * time.delta_seconds());
        life.check(*max_life);
    }
}
