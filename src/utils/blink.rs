use bevy::prelude::*;
use std::time::Duration;

use crate::prelude::GameState;

pub struct BlinkPlugin;

impl Plugin for BlinkPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(mark_blink)
                .with_system(blink)
                .with_system(blink_removed),
        );
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Blink {
    timer: Timer,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct BlinkMarker;

impl Blink {
    /// Start the blink of an entity, switching [`Visibility`] each `duration`
    pub fn new(duration: Duration) -> Self {
        Blink {
            timer: Timer::new(duration, TimerMode::Repeating),
        }
    }
}

fn mark_blink(mut commands: Commands, query: Query<Entity, Added<Blink>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(BlinkMarker);
    }
}

fn blink(time: Res<Time>, mut query: Query<(&mut Visibility, &mut Blink)>) {
    for (mut visibility, mut blink) in query.iter_mut() {
        blink.timer.tick(time.delta());
        if blink.timer.just_finished() {
            info!("blink");
            visibility.toggle();
        }
    }
}

/// Force `Visibility` to visible when `Blink` is removed
fn blink_removed(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Visibility), (With<BlinkMarker>, Without<Blink>)>,
) {
    for (entity, mut visibility) in query.iter_mut() {
        info!("blink removed : force visibility");
        visibility.is_visible = true;
        commands.entity(entity).remove::<BlinkMarker>();
    }
}
