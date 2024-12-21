mod gun;
pub use gun::Gun;

mod shuriken;
pub use shuriken::ShurikenLauncher;

mod mine;
pub use mine::MineDropper;

use super::PreUpdateAffixes;
use crate::components::*;
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct WeaponsPluginGroup;

impl PluginGroup for WeaponsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(gun::GunPlugin)
            .add(shuriken::ShurikenPlugin)
            .add(mine::MinePlugin)
            .add(weapons_plugin)
    }
}

fn weapons_plugin(app: &mut App) {
    app.register_type::<Damage>()
        .register_type::<BaseAttackSpeed>()
        .register_type::<AttackSpeed>()
        .register_type::<BaseDamageRange>()
        .register_type::<DamageRange>()
        .register_type::<AttackTimer>()
        .add_systems(
            PreUpdate,
            (
                (update_weapon_attack_speed, tick_weapon).chain(),
                update_weapon_damage_range,
            )
                .in_set(PreUpdateAffixes::Step2),
        )
        .add_observer(update_character_observers);
}

fn update_character_observers(trigger: Trigger<OnAdd, Character>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(despawn_non_projectile_damager)
        .observe(try_pierce);
}

fn update_weapon_attack_speed(
    mut weapons: Query<
        (
            &mut AttackTimer,
            &mut AttackSpeed,
            &BaseAttackSpeed,
            &Parent,
        ),
        With<Weapon>,
    >,
    characters: Query<&IncreaseAttackSpeed, With<Character>>,
) {
    for (mut timer, mut attack_speed, base_attack_speed, parent) in &mut weapons {
        if let Ok(increase) = characters.get(**parent) {
            *attack_speed = base_attack_speed * increase;
            timer.set_attack_speed(*attack_speed);
        }
    }
}

fn tick_weapon(mut weapons: Query<&mut AttackTimer, With<Weapon>>, time: Res<Time>) {
    for mut timer in &mut weapons {
        timer.tick(time.delta());
    }
}

fn update_weapon_damage_range(
    mut weapons: Query<(&mut DamageRange, &BaseDamageRange, &Parent), With<Weapon>>,
    characters: Query<(&MoreDamage, &IncreaseDamage), With<Character>>,
) {
    for (mut damage_range, base_damage_range, parent) in &mut weapons {
        if let Ok((more, increase)) = characters.get(**parent) {
            *damage_range = (base_damage_range + more) * increase;
        }
    }
}

fn despawn_non_projectile_damager(
    trigger: Trigger<HitEvent>,
    mut commands: Commands,
    damagers: Query<(), (With<Damager>, Without<Projectile>)>,
) {
    if damagers.get(trigger.entity()).is_ok() {
        commands.entity(trigger.entity()).despawn_recursive();
    }
}

fn try_pierce(
    trigger: Trigger<HitEvent>,
    mut commands: Commands,
    mut projectiles: Query<&mut PierceChance, With<Projectile>>,
) {
    if let Ok(mut pierce_chance) = projectiles.get_mut(trigger.entity()) {
        let mut rng = rand::thread_rng();
        if !pierce_chance.try_pierce(&mut rng) {
            // Didn't pierce => despawn projectile
            commands.entity(trigger.entity()).despawn();
        }
    }
}
