use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionGroups, Group};
use std::time::Duration;

pub struct InvulnerabilityPlugin;

impl Plugin for InvulnerabilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, invulnerability_finished);
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

#[derive(Component, Deref, DerefMut)]
struct InvulnerabilityAnimationTimer(Timer);

impl Default for InvulnerabilityAnimationTimer {
    fn default() -> Self {
        InvulnerabilityAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
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
    if let Ok((entity, mut collision_groups, mut invulnerable)) = query.single_mut() {
        if !invulnerable.pause {
            invulnerable.timer.tick(time.delta());
            if invulnerable.timer.just_finished() {
                collision_groups.filters |= invulnerable.filters;
                commands.entity(entity).remove::<Invulnerable>();
            }
        }
    }
}
