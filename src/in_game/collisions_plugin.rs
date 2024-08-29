use crate::components::*;
use crate::schedule::*;
use crate::utils::collision::*;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy_rapier2d::prelude::*;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                monster_hit_by_bullet,
                player_touched_by_monster,
                player_hits_bonus,
            )
                .in_set(GameRunningSet::EntityUpdate),
        );
    }
}

///
/// Monster hit by a bullet
///
fn monster_hit_by_bullet(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    q_monsters: Query<(), With<Monster>>,
    mut q_bullets: Query<(Entity, &Damage, &mut PierceChance), With<Bullet>>,
    mut monster_hit_events: EventWriter<MonsterHitEvent>,
) {
    let mut monster_hit = HashMap::new();
    let mut bullet_hit = HashSet::new();

    // apply damage
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| q_monsters.get_either(e1, e2))
        .filter_map(|(_, monster, other)| {
            q_bullets
                .get(other)
                .map(|(bullet, damage, _)| (monster, bullet, damage))
                .ok()
        })
        .for_each(|(monster, bullet, damage)| {
            *monster_hit.entry(monster).or_insert(0) += damage.0;
            bullet_hit.insert(bullet);
        });

    // try to pierce
    for bullet in bullet_hit {
        if let Ok((_, _, mut pierce)) = q_bullets.get_mut(bullet) {
            if !pierce.try_pierce() {
                // Didn't pierce => despawn bullet
                commands.entity(bullet).despawn();
            }
        }
    }

    for (entity, damage) in monster_hit.iter() {
        monster_hit_events.send(MonsterHitEvent::new(*entity, *damage));
    }
}

///
/// Player touched by monster
///
fn player_touched_by_monster(
    mut collisions: EventReader<CollisionEvent>,
    q_monsters: Query<&Damage, With<Monster>>,
    q_player: Query<(), With<Player>>,
    mut player_hit_events: EventWriter<PlayerHitEvent>,
) {
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| q_player.get_either(e1, e2))
        .filter_map(|(_, player, other)| q_monsters.get(other).map(|damage| (player, damage)).ok())
        .for_each(|(player, damage)| {
            info!("player_touched_by_monster");
            player_hit_events.send(PlayerHitEvent::new(player, **damage));
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
