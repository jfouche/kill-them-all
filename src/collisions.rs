use crate::prelude::*;
use bevy::utils::HashSet;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(invulnerability_finished)
            .add_system(monster_hit_by_bullet)
            .add_system(player_touched_by_monster);
    }
}

pub const GROUP_PLAYER: Group = Group::GROUP_1;
pub const GROUP_ENEMY: Group = Group::GROUP_2;

///
/// Monster hit by a bullet
///
fn monster_hit_by_bullet(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    q_monsters: Query<Entity, With<Monster>>,
    q_bullets: Query<Entity, With<Bullet>>,
    mut monster_hit_events: EventWriter<MonsterHitEvent>,
) {
    let mut monster_hit = HashSet::new();
    let mut bullet_hit = HashSet::new();
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
                        monster_hit.insert(monster);
                        bullet_hit.insert(bullet);
                    }
                }
            }
        });

    for bullet in bullet_hit {
        commands.entity(bullet).despawn();
    }

    for entity in monster_hit.iter() {
        monster_hit_events.send(MonsterHitEvent::new(*entity));
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

///
///
///
fn invulnerability_finished(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CollisionGroups, &mut Invulnerable)>,
) {
    if let Ok((entity, mut collision_groups, mut invulnerable)) = query.get_single_mut() {
        if invulnerable.tick_and_finished(time) {
            collision_groups.filters |= invulnerable.filters;
            commands.entity(entity).remove::<Invulnerable>();
        }
    }
}
