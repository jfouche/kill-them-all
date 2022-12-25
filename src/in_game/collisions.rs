use crate::prelude::*;
use bevy::utils::{HashMap, HashSet};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(monster_hit_by_bullet)
                .with_system(player_touched_by_monster)
                .with_system(player_hits_bonus),
        );
    }
}

pub const GROUP_PLAYER: Group = Group::GROUP_1;
pub const GROUP_ENEMY: Group = Group::GROUP_2;
pub const GROUP_BONUS: Group = Group::GROUP_3;
pub const GROUP_BULLET: Group = Group::GROUP_4;

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
        .iter()
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
            .iter()
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
