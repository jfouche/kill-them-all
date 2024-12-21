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
                player_touched_by_monster,
                player_takes_bonus,
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
    damagers: Query<&DamageRange, With<Damager>>,
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
