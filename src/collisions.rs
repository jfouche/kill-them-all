use bevy::{prelude::*, utils::HashSet};
use bevy_rapier2d::prelude::*;

use crate::components::*;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(monster_hit_by_bullet);
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
