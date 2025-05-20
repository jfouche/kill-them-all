use crate::components::character::{Character, HitEvent, MovementAction};
use crate::components::damage::{DamageOverTime, Damager, HitDamageRange};
use crate::components::monster::Monster;
use crate::components::player::Player;
use crate::schedule::GameRunningSet;
use crate::utils::collision::{start_event_filter, QueryEither};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_if_character_is_hit,
                check_if_character_is_in_damage_over_time_zone,
                player_touched_by_monster,
                stop_move_on_collision_between_characters,
            )
                .in_set(GameRunningSet::EntityUpdate),
        );
    }
}

///
/// [Character] hit by a [Damager]
///
fn check_if_character_is_hit(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    characters: Query<(), With<Character>>,
    damagers: Query<&HitDamageRange, With<Damager>>,
) {
    let mut rng = rand::rng();

    // apply damage
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| {
            let (_, character, other) = characters.get_either(e1, e2)?;
            damagers
                .get(other)
                .map(|damage_range| (character, other, damage_range))
                .ok()
        })
        .for_each(|(character, damager, damage_range)| {
            let damage = damage_range.gen(&mut rng);
            commands.trigger_targets(HitEvent { damager, damage }, character);
        });
}

///
/// Check if a [Character] starts or stops collinding with a [DamageOverTime] zone
///
/// TODO: This algo is not good as if a character is in multiple zone,
///  leaving one will stop damage over time
/// [DamageOverTime] should be a child of the [Character] to allow multiple effects
fn check_if_character_is_in_damage_over_time_zone(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    characters: Query<(), With<Character>>,
    damagers: Query<&DamageOverTime, With<Damager>>,
) {
    let get_dot = |e1, e2| {
        characters
            .get(e1)
            .and_then(|_| damagers.get(e2))
            .map(|dot| (e1, dot))
    };

    for &event in collisions.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                if let Ok((entity, &dot)) = get_dot(e1, e2).or(get_dot(e2, e1)) {
                    commands.entity(entity).insert(dot);
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if let Ok((entity, _)) = get_dot(e1, e2).or(get_dot(e2, e1)) {
                    commands.entity(entity).remove::<DamageOverTime>();
                }
            }
        }
    }
}

///
/// Player touched by monster
///
fn player_touched_by_monster(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    q_monsters: Query<&HitDamageRange, With<Monster>>,
    q_player: Query<(), With<Player>>,
) {
    let mut rng = rand::rng();
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| {
            let (_, player, other) = q_player.get_either(e1, e2)?;
            q_monsters
                .get(other)
                .map(|damage_range| (player, other, damage_range))
                .ok()
        })
        .for_each(|(player, monster, damage_range)| {
            info!("player_touched_by_monster");
            let damage = damage_range.gen(&mut rng);
            commands.trigger_targets(
                HitEvent {
                    damager: monster,
                    damage,
                },
                player,
            );
        });
}

fn stop_move_on_collision_between_characters(
    mut characters: Query<&mut MovementAction, With<Character>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for (e1, e2) in collisions.read().filter_map(start_event_filter) {
        if let Ok(mut actions) = characters.get_many_mut([*e1, *e2]) {
            actions[0].stop();
            actions[1].stop()
        }
    }
}
