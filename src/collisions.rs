use bevy::{prelude::*, utils::HashSet};
use bevy_rapier2d::prelude::*;

use crate::{components::*, player::PlayerHitEvent};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(monster_hit_by_bullet)
            .add_system(player_touched_by_monster);
    }
}

///
/// Monster hit by a bullet
///
fn monster_hit_by_bullet(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    q_monsters: Query<Entity, With<Monster>>,
    q_bullets: Query<Entity, With<Bullet>>,
) {
    let mut entities_destroyed = HashSet::new();
    collisions
        .iter()
        .filter_map(|e| match e {
            CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
            _ => None,
        })
        .for_each(|(&e1, &e2)| {
            for monster in q_monsters.iter() {
                for bullet in q_bullets.iter() {
                    if (e1 == monster && e2 == bullet) || (e1 == bullet && e2 == monster) {
                        entities_destroyed.insert(e1);
                        entities_destroyed.insert(e2);
                    }
                }
            }
        });

    for entity in entities_destroyed.iter() {
        commands.entity(*entity).despawn();
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
        .iter()
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
