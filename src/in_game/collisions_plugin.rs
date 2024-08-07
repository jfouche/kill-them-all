use crate::components::*;
use crate::schedule::*;
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
                .in_set(InGameSet::CollisionDetection),
        );
    }
}

///
/// Monster hit by a bullet
///
fn monster_hit_by_bullet(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    q_monsters: Query<Entity, With<Monster>>,
    q_bullets: Query<(Entity, &Damage), With<Bullet>>,
    mut monster_hit_events: EventWriter<MonsterHitEvent>,
) {
    let mut monster_hit = HashMap::new();
    let mut bullet_hit = HashSet::new();
    collisions
        .read()
        .filter_map(|e| match e {
            CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
            _ => None,
        })
        .for_each(|(&e1, &e2)| {
            for monster in q_monsters.iter() {
                for (bullet, damage) in q_bullets.iter() {
                    if (e1 == monster && e2 == bullet) || (e1 == bullet && e2 == monster) {
                        *monster_hit.entry(monster).or_insert(0) += damage.0;
                        bullet_hit.insert(bullet);
                    }
                }
            }
        });

    for bullet in bullet_hit {
        commands.entity(bullet).despawn();
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
    q_monsters: Query<Entity, With<Monster>>,
    q_player: Query<Entity, With<Player>>,
    mut player_hit_events: EventWriter<PlayerHitEvent>,
) {
    collisions
        .read()
        .filter_map(|e| match e {
            CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
            _ => None,
        })
        .for_each(|(&e1, &e2)| {
            if let Ok(player) = q_player.get_single() {
                for monster in q_monsters.iter() {
                    if (e1 == player && e2 == monster) || (e1 == monster && e2 == player) {
                        warn!("player_touched_by_monster");
                        player_hit_events.send(PlayerHitEvent::new(player));
                    }
                }
            }
        });
}

///
/// Player takes bonus
///
fn player_hits_bonus(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut q_player: Query<(Entity, &mut Money), With<Player>>,
    q_bonus: Query<Entity, With<Bonus>>,
) {
    if let Ok((player, mut money)) = q_player.get_single_mut() {
        collisions
            .read()
            .filter_map(|e| match e {
                CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
                _ => None,
            })
            .for_each(|(&e1, &e2)| {
                for bonus in q_bonus.iter() {
                    if (e1 == bonus && e2 == player) || (e1 == player && e2 == bonus) {
                        money.0 += 1;
                        commands.entity(bonus).despawn();
                    }
                }
            });
    }
}
