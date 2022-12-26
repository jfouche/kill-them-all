use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionGroups, Group};
use std::time::Duration;

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
    pause: bool,
}

impl Invulnerable {
    pub fn new(duration: Duration, filters: Group) -> Self {
        Invulnerable {
            timer: Timer::new(duration, TimerMode::Once),
            filters,
            pause: false,
        }
    }

    /// pause invulnerability
    pub fn pause(&mut self, pause: bool) {
        self.pause = pause;
    }
}

///
/// [`Invulnerable`] starts
///
fn invulnerability_started(query: Query<Entity, Added<Invulnerable>>) {
    for _entity in query.iter() {
        warn!("invulnerability_started");
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
        if !invulnerable.pause {
            invulnerable.timer.tick(time.delta());
            if invulnerable.timer.just_finished() {
                warn!("invulnerability_finished");
                collision_groups.filters |= invulnerable.filters;
                commands.entity(entity).remove::<Invulnerable>();
            }
        }
    }
}
