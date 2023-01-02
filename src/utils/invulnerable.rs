use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionGroups, Group};
use std::time::Duration;

use crate::prelude::Blink;

pub struct InvulnerabilityPlugin;

impl Plugin for InvulnerabilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(invulnerability_started)
            .add_system(invulnerability_finished);
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Invulnerable {
    pub filters: Group,
    timer: Timer,
}

impl Invulnerable {
    pub fn new(duration: Duration, filters: Group) -> Self {
        Invulnerable {
            timer: Timer::new(duration, TimerMode::Once),
            filters,
        }
    }
}

///
/// [`Invulnerable`] starts
///
fn invulnerability_started(mut commands: Commands, query: Query<Entity, Added<Invulnerable>>) {
    for entity in query.iter() {
        info!("invulnerability_started");
        commands
            .entity(entity)
            .insert(Blink::new(Duration::from_millis(250)));
    }
}

///
/// [`Invulnerable`] finishes
///
fn invulnerability_finished(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CollisionGroups, &mut Invulnerable)>,
) {
    if let Ok((entity, mut collision_groups, mut invulnerable)) = query.get_single_mut() {
        invulnerable.timer.tick(time.delta());
        if invulnerable.timer.just_finished() {
            info!("invulnerability_finished");
            collision_groups.filters |= invulnerable.filters;
            commands.entity(entity).remove::<(Invulnerable, Blink)>();
        }
    }
}
