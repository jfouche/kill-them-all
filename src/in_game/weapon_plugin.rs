use crate::components::*;
use bevy::prelude::*;

use super::PreUpdateAffixes;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Damage>()
            .register_type::<BaseAttackSpeed>()
            .register_type::<AttackSpeed>()
            .register_type::<BaseHitDamageRange>()
            .register_type::<HitDamageRange>()
            .register_type::<BaseDamageOverTime>()
            .register_type::<DamageOverTime>()
            .register_type::<AttackTimer>()
            .add_systems(
                PreUpdate,
                (
                    (update_weapon_attack_speed, tick_weapon).chain(),
                    update_weapon_hit_damage_range,
                    update_weapon_damage_over_time,
                )
                    .in_set(PreUpdateAffixes::Step2),
            );
    }
}

fn update_weapon_attack_speed(
    mut weapons: Query<
        (
            &mut AttackTimer,
            &mut AttackSpeed,
            &BaseAttackSpeed,
            &Parent,
        ),
        Or<(With<Skill>, With<Weapon>)>, // TODO: move skill ?
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
