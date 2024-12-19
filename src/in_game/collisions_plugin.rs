use crate::components::*;
use crate::schedule::*;
use crate::utils::collision::*;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy_rapier2d::prelude::*;
use rand::thread_rng;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                character_hit_by_ammo,
                player_touched_by_monster,
                player_hits_bonus,
            )
                .in_set(GameRunningSet::EntityUpdate),
        )
        .add_observer(update_character_observers);
    }
}

fn update_character_observers(trigger: Trigger<OnAdd, Character>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(despawn_non_projectile_ammo)
        .observe(try_pierce);
}

///
/// [Character] hit by an [Ammo]
///
fn character_hit_by_ammo(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    characters: Query<(), With<Character>>,
    ammos: Query<(Entity, &DamageRange), With<Ammo>>,
) {
    let mut characters_hits = HashMap::new();
    let mut ammo_hits = HashSet::new();
    let mut rng = rand::thread_rng();

    // apply damage
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| characters.get_either(e1, e2))
        .filter_map(|(_, character, other)| {
            ammos
                .get(other)
                .map(|(ammo, damage_range)| (character, ammo, damage_range))
                .ok()
        })
        .for_each(|(character, ammo, damage_range)| {
            *characters_hits.entry(character).or_default() += damage_range.gen(&mut rng);
            ammo_hits.insert(ammo);
        });

    for (character_entity, damage) in characters_hits.iter() {
        commands.trigger_targets(HitEvent { damage: *damage }, *character_entity);
    }
}

fn despawn_non_projectile_ammo(
    trigger: Trigger<HitEvent>,
    mut commands: Commands,
    ammos: Query<(), (With<Ammo>, Without<Projectile>)>,
) {
    if ammos.get(trigger.entity()).is_ok() {
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

///
/// Player touched by monster
///
fn player_touched_by_monster(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    q_monsters: Query<&DamageRange, With<Monster>>,
    q_player: Query<(), With<Player>>,
) {
    let mut rng = thread_rng();
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| q_player.get_either(e1, e2))
        .filter_map(|(_, player, other)| {
            q_monsters
                .get(other)
                .map(|damage_range| (player, damage_range))
                .ok()
        })
        .for_each(|(player, damage_range)| {
            info!("player_touched_by_monster");
            let damage = damage_range.gen(&mut rng);
            commands.trigger_targets(HitEvent { damage }, player);
        });
}

///
/// Player takes bonus
///
fn player_hits_bonus(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut q_player: Query<&mut Money, With<Player>>,
    q_bonus: Query<(), With<Bonus>>,
) {
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| q_bonus.get_either(e1, e2))
        .for_each(|(_, bonus, other)| {
            if let Ok(mut money) = q_player.get_mut(other) {
                **money += 1;
                commands.entity(bonus).despawn();
            }
        });
}
