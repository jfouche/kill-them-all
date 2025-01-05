use crate::components::*;
use crate::schedule::*;
use crate::utils::collision::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::*;
use rand::thread_rng;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_if_character_is_hit,
                check_if_character_is_in_damage_over_time_zone,
                player_touched_by_monster,
                player_takes_bonus,
                stop_move_on_collision,
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
    let mut characters_hits = HashMap::new();
    let mut rng = rand::thread_rng();

    // apply damage
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| {
            let (_, character, other) = characters.get_either(e1, e2)?;
            damagers
                .get(other)
                .map(|damage_range| (character, damage_range))
                .ok()
        })
        .for_each(|(character, damage_range)| {
            *characters_hits.entry(character).or_default() += damage_range.gen(&mut rng);
        });

    for (character_entity, damage) in characters_hits.iter() {
        commands.trigger_targets(HitEvent { damage: *damage }, *character_entity);
    }
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
    let mut rng = thread_rng();
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| {
            let (_, player, other) = q_player.get_either(e1, e2)?;
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
fn player_takes_bonus(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut players: Query<(), With<Player>>,
    bonuses: Query<&Bonus>,
) {
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| {
            let (_, bonus_entity, other_entity) = bonuses.get_either(e1, e2)?;
            players.get_mut(other_entity).map(|_| bonus_entity).ok()
        })
        .for_each(|bonus_entity| {
            commands.queue(TakeBonusCommand(bonus_entity));
        });
}

fn stop_move_on_collision(
    mut characters: Query<(Entity, &mut NextPosition), With<Character>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for (character, mut next_pos) in &mut characters {
        if collisions
            .read()
            .filter_map(start_event_filter)
            .any(|(e1, e2)| character == *e1 || character == *e2)
        {
            next_pos.stop();
        }
    }
}
