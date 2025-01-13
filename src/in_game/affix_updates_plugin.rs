use super::game_is_running;
use crate::components::*;
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
enum PreUpdateAffixes {
    Characters,
    Weapons,
    Skills,
}

/// Manage affixes update
pub struct AffixUpdatesPlugin;

impl Plugin for AffixUpdatesPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PreUpdate,
            (
                PreUpdateAffixes::Characters,
                PreUpdateAffixes::Weapons,
                PreUpdateAffixes::Skills,
            )
                .chain()
                .run_if(game_is_running),
        )
        .add_systems(
            PreUpdate,
            (
                (
                    (update_armour::<Equipment>, update_armour::<Character>).chain(),
                    (update_increase_attack_speed, update_weapon_attack_speed).chain(),
                    update_pierce_chance,
                    update_increase_area_of_effect,
                    (update_more_damage, update_increase_damage).chain(),
                )
                    .in_set(PreUpdateAffixes::Characters),
                (
                    (update_max_life, update_life_regen).chain(),
                    update_movement_speed,
                )
                    .run_if(game_is_running),
            ),
        )
        .add_systems(
            PreUpdate,
            (
                (update_weapon_attack_speed, tick_weapon).chain(),
                update_weapon_hit_damage_range,
                update_weapon_damage_over_time,
            )
                .in_set(PreUpdateAffixes::Weapons),
        )
        .add_observer(fix_life);
    }
}

/// Fix the life when adding [MoreLife] or [IncreaseMaxLife] affixes
fn fix_life(
    trigger: Trigger<OnAdd, Parent>,
    affixes: Query<(&Parent, Option<&MoreLife>, Option<&IncreaseMaxLife>)>,
    mut characters: Query<&mut Life, With<Character>>,
) {
    if let Ok((parent, more, increase)) = affixes.get(trigger.entity()) {
        if let Ok(mut life) = characters.get_mut(**parent) {
            if let Some(more) = more {
                life.regenerate(**more);
            }
            if let Some(increase) = increase {
                let more = **life * **increase / 100.;
                life.regenerate(more);
            }
        }
    }
}

/// [Armour] = sum([Armour]) + sum ([MoreArmour])
fn update_armour<T: Component>(
    mut armoreds: Query<&mut Armour, With<T>>,
    armours: Query<(&Armour, &Parent), Without<T>>,
    more_armours: Query<(&MoreArmour, &Parent), Without<T>>,
) {
    for mut armour in &mut armoreds {
        **armour = 0.;
    }
    for (affix_armour, parent) in &armours {
        if let Ok(mut armour) = armoreds.get_mut(**parent) {
            **armour += **affix_armour;
        }
    }
    for (more_armour, parent) in &more_armours {
        if let Ok(mut armour) = armoreds.get_mut(**parent) {
            **armour += **more_armour;
        }
    }
}

/// [MaxLife] = ([BaseLife] + sum([MoreLife])) * sum([IncreaseMaxLife]) %
fn update_max_life(
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

/// [IncreaseAttackSpeed] = sum([IncreaseAttackSpeed])
fn update_increase_attack_speed(
    mut characters: Query<&mut IncreaseAttackSpeed, With<Character>>,
    affixes: Query<(&IncreaseAttackSpeed, &Parent), Without<Character>>,
) {
    for mut character_incr_attack_speed in &mut characters {
        // Attack speed
        **character_incr_attack_speed = 0.;
    }
    for (incr_attack_speed, parent) in &affixes {
        if let Ok(mut character_incr_attack_speed) = characters.get_mut(**parent) {
            **character_incr_attack_speed += **incr_attack_speed;
        }
    }
}

/// [PierceChance] = sum([PierceChance])
fn update_pierce_chance(
    mut characters: Query<&mut PierceChance, With<Character>>,
    mut affixes: Query<(&mut PierceChance, &Parent), Without<Character>>,
) {
    for mut char_pierce_chance in &mut characters {
        **char_pierce_chance = 0.;
    }

    for (pierce_chance, parent) in &mut affixes {
        if let Ok(mut char_pierce_chance) = characters.get_mut(**parent) {
            **char_pierce_chance += **pierce_chance;
        }
    }
}

/// [MoreDamage] = sum([MoreDamage])
fn update_more_damage(
    mut characters: Query<&mut MoreDamage, With<Character>>,
    mut affixes: Query<(&mut MoreDamage, &Parent), Without<Character>>,
) {
    for mut more_damage in &mut characters {
        **more_damage = 0.;
    }
    for (more_damage, parent) in &mut affixes {
        if let Ok(mut char_more_damage) = characters.get_mut(**parent) {
            **char_more_damage += **more_damage;
        }
    }
}

/// [IncreaseDamage] = sum([IncreaseDamage])
fn update_increase_damage(
    mut characters: Query<&mut IncreaseDamage, With<Character>>,
    mut affixes: Query<(&mut IncreaseDamage, &Parent), Without<Character>>,
) {
    for mut incr_damage in &mut characters {
        **incr_damage = 0.;
    }
    for (incr_damage, parent) in &mut affixes {
        if let Ok(mut char_incr_damage) = characters.get_mut(**parent) {
            **char_incr_damage += **incr_damage;
        }
    }
}

/// [IncreaseAreaOfEffect] = sum([IncreaseAreaOfEffect])
fn update_increase_area_of_effect(
    mut characters: Query<&mut IncreaseAreaOfEffect, With<Character>>,
    mut affixes: Query<(&mut IncreaseAreaOfEffect, &Parent), Without<Character>>,
) {
    for mut incr_aoe in &mut characters {
        **incr_aoe = 0.;
    }
    for (incr_aoe, parent) in &mut affixes {
        if let Ok(mut char_incr_aoe) = characters.get_mut(**parent) {
            **char_incr_aoe += **incr_aoe;
        }
    }
}

/// Weapon [AttackSpeed] = [BaseAttackSpeed] * sum([IncreaseAttackSpeed])
///
/// Update also the [AttackTimer] based on the new [AttackSpeed].
fn update_weapon_attack_speed(
    mut weapons: Query<
        (
            &mut AttackTimer,
            &mut AttackSpeed,
            &BaseAttackSpeed,
            &Parent,
        ),
        Or<(With<Skill>, With<Weapon>)>,
    >,
    characters: Query<&IncreaseAttackSpeed, With<Character>>,
) {
    for (mut timer, mut attack_speed, base_attack_speed, parent) in &mut weapons {
        if let Ok(increase) = characters.get(**parent) {
            *attack_speed = base_attack_speed.attack_speed(increase);
            timer.set_attack_speed(*attack_speed);
        }
    }
}

fn tick_weapon(mut weapons: Query<&mut AttackTimer, With<Weapon>>, time: Res<Time>) {
    for mut timer in &mut weapons {
        timer.tick(time.delta());
    }
}

fn update_weapon_hit_damage_range(
    mut weapons: Query<(&mut HitDamageRange, &BaseHitDamageRange, &Parent), With<Weapon>>,
    characters: Query<(&MoreDamage, &IncreaseDamage), With<Character>>,
) {
    for (mut damage_range, base_hit_damage_range, parent) in &mut weapons {
        if let Ok((more, increase)) = characters.get(**parent) {
            *damage_range = base_hit_damage_range.damage_range(more, increase);
        }
    }
}

fn update_weapon_damage_over_time(
    mut weapons: Query<
        (&mut DamageOverTime, &BaseDamageOverTime, &Parent),
        Or<(With<Skill>, With<Weapon>)>, // TODO: move skill ?
    >,
    characters: Query<(&MoreDamage, &IncreaseDamage), With<Character>>,
) {
    for (mut damage_over_time, base, parent) in &mut weapons {
        if let Ok((more, increase)) = characters.get(**parent) {
            *damage_over_time = base.damage_over_time(more, increase);
        }
    }
}
