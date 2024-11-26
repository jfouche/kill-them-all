use super::weapons::Bullet;
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
                monster_hit_by_ammo,
                player_touched_by_monster,
                player_hits_bonus,
            )
                .in_set(GameRunningSet::EntityUpdate),
        );
    }
}

///
/// Monster hit by an [Ammo]
///
fn monster_hit_by_ammo(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    monsters: Query<(), With<Monster>>,
    mut ammos: Query<(Entity, &Damage, &mut PierceChance), With<Ammo>>,
) {
    let mut monster_hits = HashMap::new();
    let mut ammo_hits = HashSet::new();

    // apply damage
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| monsters.get_either(e1, e2))
        .filter_map(|(_, monster, other)| {
            ammos
                .get(other)
                .map(|(ammo, damage, _)| (monster, ammo, damage))
                .ok()
        })
        .for_each(|(monster, ammo, damage)| {
            *monster_hits.entry(monster).or_insert(0.) += **damage;
            ammo_hits.insert(ammo);
        });

    // try to pierce
    for ammo in ammo_hits {
        if let Ok((_, _, mut pierce)) = ammos.get_mut(ammo) {
            if !pierce.try_pierce() {
                // Didn't pierce => despawn bullet
                commands.entity(ammo).despawn();
            }
        }
    }

    for (entity, damage) in monster_hits.iter() {
        commands.trigger_targets(
            HitEvent {
                damage: Damage(*damage),
            },
            *entity,
        );
    }
}

///
/// Player touched by monster
///
fn player_touched_by_monster(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    q_monsters: Query<&Damage, With<Monster>>,
    q_player: Query<(), With<Player>>,
) {
    collisions
        .read()
        .filter_map(start_event_filter)
        .filter_map(|(&e1, &e2)| q_player.get_either(e1, e2))
        .filter_map(|(_, player, other)| q_monsters.get(other).map(|damage| (player, damage)).ok())
        .for_each(|(player, damage)| {
            info!("player_touched_by_monster");
            commands.trigger_targets(HitEvent { damage: *damage }, player);
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
