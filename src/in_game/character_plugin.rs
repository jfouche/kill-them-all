use crate::{components::*, schedule::*};
use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HitEvent>()
            .add_event::<LooseLifeEvent>()
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
            .register_type::<AffixesLabels>()
            .add_systems(Startup, register_hooks)
            .add_systems(
                PreUpdate,
                (
                    update_armour,
                    update_life,
                    update_life_regen,
                    update_movement_speed,
                    (update_increase_attack_speed, update_weapon_attack_speed).chain(),
                    update_pierce_chance,
                )
                    .run_if(game_is_running),
            )
            .add_systems(Update, regen_life.in_set(GameRunningSet::EntityUpdate));
    }
}

fn register_hooks(world: &mut World) {
    world
        .register_component_hooks::<Character>()
        .on_add(|mut world, entity, _component_id| {
            world
                .commands()
                .entity(entity)
                .observe(mitigate_damage_on_hit)
                .observe(loose_life)
                .observe(despawn_character_on_death);
        });
}

fn mitigate_damage_on_hit(
    trigger: Trigger<HitEvent>,
    mut commands: Commands,
    characters: Query<&Armour, With<Character>>,
) {
    if let Ok(armour) = characters.get(trigger.entity()) {
        let damage = armour.mitigate(trigger.event().damage);
        info!("trigger_take_hit: damage: {:.1}", *damage);
        if *damage > 0. {
            commands.trigger_targets(LooseLifeEvent(damage), trigger.entity());
        }
    }
}

fn loose_life(
    trigger: Trigger<LooseLifeEvent>,
    mut commands: Commands,
    mut characters: Query<&mut Life, With<Character>>,
) {
    if let Ok(mut life) = characters.get_mut(trigger.entity()) {
        info!("loose_life : {:.1} - {:.1}", **life, ***trigger.event());
        life.hit(**trigger.event());
        if life.is_dead() {
            commands.trigger_targets(CharacterDyingEvent, trigger.entity());
        }
    }
}

fn despawn_character_on_death(trigger: Trigger<CharacterDiedEvent>, mut commands: Commands) {
    commands.entity(trigger.entity()).despawn_recursive();
}

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

/// [MaxLife] = ([BaseLife] + sum([MoreLife])) * sum([IncreaseMaxLife]) %
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

/// Weapon [AttackSpeed] = [BaseAttackSpeed] * sum([IncreaseAttackSpeed])
fn update_weapon_attack_speed(
    mut weapons: Query<(&mut AttackSpeed, &BaseAttackSpeed, &Parent)>,
    characters: Query<&IncreaseAttackSpeed, With<Character>>,
) {
    for (mut attack_speed, base_attack_speed, parent) in &mut weapons {
        **attack_speed = **base_attack_speed;
        if let Ok(incr) = characters.get(**parent) {
            **attack_speed *= **incr;
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

fn regen_life(mut query: Query<(&mut Life, &MaxLife, &LifeRegen)>, time: Res<Time>) {
    for (mut life, max_life, regen) in &mut query {
        let life_per_sec = **max_life * (**regen / 100.);
        life.regenerate(life_per_sec * time.delta_secs());
        life.check(*max_life);
    }
}
