use crate::{
    components::{
        affix::{
            Armour, ArmourUpdateQuery, IncreaseAreaOfEffect, IncreaseAttackSpeed, IncreaseDamage,
            IncreaseMaxLife, IncreaseMovementSpeed, LifeRegen, MoreArmour, MoreDamage, MoreLife,
            PierceChance,
        },
        character::{BaseLife, BaseMovementSpeed, Character, Life, MaxLife, MovementSpeed},
        damage::{BaseDamageOverTime, BaseHitDamageRange, DamageOverTime, HitDamageRange},
        equipment::{
            weapon::{AttackSpeed, AttackTimer, BaseAttackSpeed},
            Equipment, Weapon,
        },
        skills::Skill,
    },
    schedule::game_is_running,
};
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
enum PreUpdateAffixes {
    LocalEquipment,
    Characters,
    Skills,
}

/// Manage affixes update
pub struct AffixUpdatesPlugin;

impl Plugin for AffixUpdatesPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PreUpdate,
            (
                PreUpdateAffixes::LocalEquipment,
                PreUpdateAffixes::Characters,
                PreUpdateAffixes::Skills,
            )
                .chain()
                .run_if(game_is_running),
        )
        .add_systems(
            PreUpdate,
            (
                (
                    update_equipment_armour,
                    update_weapon_attack_speed,
                    update_weapon_hit_damage_range,
                )
                    .in_set(PreUpdateAffixes::LocalEquipment),
                (
                    (update_max_life, update_life_regen).chain(),
                    update_character_armour,
                    update_character_movement_speed,
                    update_character_increase_attack_speed,
                    update_character_pierce_chance,
                    update_character_more_damage,
                    update_character_increase_damage,
                    update_increase_area_of_effect,
                )
                    .in_set(PreUpdateAffixes::Characters),
                (
                    update_skill_damage_over_time,
                    update_skill_attack_speed,
                    update_skill_hit_damage_range,
                )
                    .in_set(PreUpdateAffixes::Skills),
                tick_attack_skill.after(PreUpdateAffixes::Skills),
            ),
        )
        .add_observer(fix_life);
    }
}

/// Fix the life when adding [MoreLife] or [IncreaseMaxLife] affixes
fn fix_life(
    trigger: Trigger<OnAdd, ChildOf>,
    affixes: Query<(&ChildOf, Option<&MoreLife>, Option<&IncreaseMaxLife>)>,
    mut characters: Query<(&mut Life, &MaxLife), With<Character>>,
) {
    if let Ok((child_of, more, increase)) = affixes.get(trigger.target()) {
        if let Ok((mut life, &max_life)) = characters.get_mut(child_of.parent()) {
            if let Some(more) = more {
                life.regenerate(**more, max_life);
            }
            if let Some(increase) = increase {
                let more = **life * **increase / 100.;
                life.regenerate(more, max_life);
            }
        }
    }
}

fn update_equipment_armour(mut equipments: Query<ArmourUpdateQuery, With<Equipment>>) {
    for mut armour in &mut equipments {
        armour.update();
    }
}

/// Weapon [AttackSpeed] = [BaseAttackSpeed] * [IncreaseAttackSpeed]
///
/// Update also the [AttackTimer] based on the new [AttackSpeed].
fn update_weapon_attack_speed(
    mut weapons: Query<
        (
            &mut AttackSpeed,
            &mut AttackTimer,
            &BaseAttackSpeed,
            Option<&IncreaseAttackSpeed>,
        ),
        With<Weapon>,
    >,
) {
    for (mut attack_speed, mut timer, base, incr) in &mut weapons {
        attack_speed.init(base);
        if let Some(incr) = incr {
            attack_speed.increase(incr);
        }
        timer.set_attack_speed(*attack_speed);
    }
}

/// [Armour] = sum([Equipment] [Armour]) + sum ([MoreArmour] affixes)
fn update_character_armour(
    mut characters: Query<&mut Armour, With<Character>>,
    equipment_armours: Query<(&Armour, &ChildOf), (With<Equipment>, Without<Character>)>,
    more_armours: Query<(&MoreArmour, &ChildOf), (Without<Equipment>, Without<Character>)>,
) {
    for mut armour in &mut characters {
        armour.reset();
    }
    for (eqp_armour, child_of) in &equipment_armours {
        if let Ok(mut armour) = characters.get_mut(child_of.parent()) {
            armour.add(eqp_armour);
        }
    }
    for (more_armour, child_of) in &more_armours {
        if let Ok(mut armour) = characters.get_mut(child_of.parent()) {
            armour.more(more_armour);
        }
    }
}

/// [MaxLife] = ([BaseLife] + sum([MoreLife])) * sum([IncreaseMaxLife]) %
fn update_max_life(
    mut characters: Query<(&BaseLife, &mut MaxLife, &mut IncreaseMaxLife), With<Character>>,
    more_affixes: Query<(&MoreLife, &ChildOf), Without<Character>>,
    incr_affixes: Query<(&IncreaseMaxLife, &ChildOf), Without<Character>>,
) {
    for (base_life, mut max_life, mut incr_life) in &mut characters {
        max_life.init(base_life);
        incr_life.reset();
    }
    for (more_life, child_of) in &more_affixes {
        if let Ok((_base_life, mut max_life, _incr_life)) = characters.get_mut(child_of.parent()) {
            max_life.more(more_life);
        }
    }
    for (incr_life, child_of) in &incr_affixes {
        if let Ok((_base_life, _life, mut incr_char_life)) = characters.get_mut(child_of.parent()) {
            incr_char_life.add(incr_life);
        }
    }

    for (_base_life, mut max_life, incr_life) in &mut characters {
        max_life.increase(&incr_life);
    }
}

/// [LifeRegen] = sum([LifeRegen])
fn update_life_regen(
    mut characters: Query<&mut LifeRegen, With<Character>>,
    affixes: Query<(&LifeRegen, &ChildOf), Without<Character>>,
) {
    for mut char_life_regen in &mut characters {
        char_life_regen.reset();
    }
    for (life_regen, child_of) in &affixes {
        if let Ok(mut char_life_regen) = characters.get_mut(child_of.parent()) {
            char_life_regen.add(life_regen);
        }
    }
}

/// [MovementSpeed] = [BaseMovementSpeed] * sum([IncreaseMovementSpeed]) %
fn update_character_movement_speed(
    mut characters: Query<
        (
            &BaseMovementSpeed,
            &mut MovementSpeed,
            &mut IncreaseMovementSpeed,
        ),
        With<Character>,
    >,
    affixes: Query<(&IncreaseMovementSpeed, &ChildOf), Without<Character>>,
) {
    for (base_move_speed, mut move_speed, mut incr_move_speed) in &mut characters {
        move_speed.init(base_move_speed);
        incr_move_speed.reset();
    }
    for (incr_move_speed, child_of) in &affixes {
        if let Ok((_, _, mut char_incr_move_speed)) = characters.get_mut(child_of.parent()) {
            char_incr_move_speed.add(incr_move_speed);
        }
    }
    for (_, mut move_speed, incr_move_speed) in &mut characters {
        move_speed.increase(&incr_move_speed);
    }
}

/// [IncreaseAttackSpeed] = sum([IncreaseAttackSpeed])
fn update_character_increase_attack_speed(
    mut characters: Query<&mut IncreaseAttackSpeed, With<Character>>,
    affixes: Query<(&IncreaseAttackSpeed, &ChildOf), (Without<Character>, Without<Weapon>)>,
) {
    for mut character_incr_attack_speed in &mut characters {
        character_incr_attack_speed.reset();
    }
    for (incr_attack_speed, child_of) in &affixes {
        if let Ok(mut character_incr_attack_speed) = characters.get_mut(child_of.parent()) {
            character_incr_attack_speed.add(incr_attack_speed);
        }
    }
}

/// [PierceChance] = sum([PierceChance])
fn update_character_pierce_chance(
    mut characters: Query<&mut PierceChance, With<Character>>,
    affixes: Query<(&PierceChance, &ChildOf), Without<Character>>,
) {
    for mut char_pierce_chance in &mut characters {
        char_pierce_chance.reset();
    }

    for (pierce_chance, child_of) in &affixes {
        if let Ok(mut char_pierce_chance) = characters.get_mut(child_of.parent()) {
            char_pierce_chance.add(pierce_chance);
        }
    }
}

/// [MoreDamage] = sum([MoreDamage])
fn update_character_more_damage(
    mut characters: Query<&mut MoreDamage, With<Character>>,
    affixes: Query<(&MoreDamage, &ChildOf), (Without<Character>, Without<Weapon>)>,
) {
    for mut more_damage in &mut characters {
        more_damage.reset();
    }
    for (more_damage, child_of) in &affixes {
        if let Ok(mut char_more_damage) = characters.get_mut(child_of.parent()) {
            char_more_damage.add(more_damage);
        }
    }
}

/// [IncreaseDamage] = sum([IncreaseDamage])
fn update_character_increase_damage(
    mut characters: Query<&mut IncreaseDamage, With<Character>>,
    affixes: Query<(&IncreaseDamage, &ChildOf), (Without<Character>, Without<Weapon>)>,
) {
    for mut incr_damage in &mut characters {
        incr_damage.reset();
    }
    for (incr_damage, child_of) in &affixes {
        if let Ok(mut char_incr_damage) = characters.get_mut(child_of.parent()) {
            char_incr_damage.add(incr_damage);
        }
    }
}

/// [IncreaseAreaOfEffect] = sum([IncreaseAreaOfEffect])
fn update_increase_area_of_effect(
    mut characters: Query<&mut IncreaseAreaOfEffect, With<Character>>,
    affixes: Query<(&IncreaseAreaOfEffect, &ChildOf), Without<Character>>,
) {
    for mut incr_aoe in &mut characters {
        incr_aoe.reset();
    }
    for (incr_aoe, child_of) in &affixes {
        if let Ok(mut char_incr_aoe) = characters.get_mut(child_of.parent()) {
            char_incr_aoe.add(incr_aoe);
        }
    }
}

fn tick_attack_skill(mut skills: Query<&mut AttackTimer, With<Skill>>, time: Res<Time>) {
    for mut timer in &mut skills {
        timer.tick(time.delta());
    }
}

/// [HitDamageRange] = ([BaseHitDamageRange] + [MoreDamage]) * [IncreaseDamage]%
fn update_weapon_hit_damage_range(
    mut weapons: Query<
        (
            &mut HitDamageRange,
            &BaseHitDamageRange,
            Option<&MoreDamage>,
            Option<&IncreaseDamage>,
        ),
        With<Weapon>,
    >,
) {
    for (mut damage_range, base, more, increase) in &mut weapons {
        damage_range.init(base);
        if let Some(more) = more {
            damage_range.more(more);
        }
        if let Some(increase) = increase {
            damage_range.increase(increase);
        }
    }
}

fn update_skill_damage_over_time(
    mut weapons: Query<(&mut DamageOverTime, &BaseDamageOverTime, &ChildOf), With<Skill>>,
    characters: Query<(&MoreDamage, &IncreaseDamage), With<Character>>,
) {
    for (mut damage_over_time, base, child_of) in &mut weapons {
        if let Ok((more, increase)) = characters.get(child_of.parent()) {
            *damage_over_time = base.damage_over_time(more, increase);
        }
    }
}

/// [Skill]'s [AttackSpeed] = [Skill]'s [BaseAttackSpeed] * [Weapon]'s [AttackSpeed] * [Character] [IncreaseAttackSpeed]
fn update_skill_attack_speed(
    mut skills: Query<
        (
            &mut AttackSpeed,
            &BaseAttackSpeed,
            &mut AttackTimer,
            &ChildOf,
        ),
        With<Skill>,
    >,
    weapons: Query<(&AttackSpeed, &IncreaseAttackSpeed, &ChildOf), (With<Weapon>, Without<Skill>)>,
    characters: Query<&IncreaseAttackSpeed, With<Character>>,
) {
    for (mut skill_attack_speed, base, mut timer, child_of) in &mut skills {
        skill_attack_speed.init(base);
        if let Some((_weapon_attack_speed, increase)) = weapons
            .iter()
            .find(|(_, _, co)| *co == child_of)
            .map(|(val, incr, _)| (val, incr))
        {
            skill_attack_speed.increase(increase);
        }
        if let Ok(increase) = characters.get(child_of.parent()) {
            skill_attack_speed.increase(increase);
        }
        timer.set_attack_speed(*skill_attack_speed);
    }
}

/// [Skill]'s [HitDamageRange] = ([Weapon]'s [HitDamageRange] + [Character]'s [MoreDamage]) * [Character]'s [IncreaseDamage]
fn update_skill_hit_damage_range(
    mut skills: Query<(&mut HitDamageRange, &BaseHitDamageRange, &ChildOf), With<Skill>>,
    weapons: Query<(&HitDamageRange, &ChildOf), (With<Weapon>, Without<Skill>)>,
    characters: Query<(Option<&MoreDamage>, Option<&IncreaseDamage>), With<Character>>,
) {
    for (mut skill_damage_range, base, child_of) in &mut skills {
        skill_damage_range.init(base);
        if let Some(weapon_damage_range) = weapons
            .iter()
            .find(|(_, co)| *co == child_of)
            .map(|(val, _)| val)
        {
            skill_damage_range.add(weapon_damage_range);
            if let Ok((more, increase)) = characters.get(child_of.parent()) {
                if let Some(more) = more {
                    skill_damage_range.more(more);
                }
                if let Some(increase) = increase {
                    skill_damage_range.increase(increase);
                }
            }
        }
    }
}
