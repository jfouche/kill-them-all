use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use std::time::Duration;

pub struct BlinkPlugin;

impl Plugin for BlinkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, blink);
    }
}

pub struct Blink {
    timer: Timer,
    pause: bool,
}

impl Component for Blink {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_remove(|mut world, entity, _component_id| {
            if let Some(mut visibility) = world.get_mut::<Visibility>(entity) {
                *visibility = Visibility::Inherited;
            }
        });
    }
}

impl Blink {
    /// Start the blink of an entity, switching [`Visibility`] each `duration`
    pub fn new(duration: Duration) -> Self {
        Blink {
            timer: Timer::new(duration, TimerMode::Repeating),
            pause: false,
        }
    }

    /// pause the blink
    pub fn pause(&mut self, pause: bool) {
        self.pause = pause;
    }
}

fn blink(time: Res<Time>, mut query: Query<(&mut Visibility, &mut Blink)>) {
    for (mut visibility, mut blink) in query.iter_mut() {
        if !blink.pause {
            blink.timer.tick(time.delta());
            if blink.timer.just_finished() {
                if *visibility == Visibility::Inherited {
                    *visibility = Visibility::Hidden;
                } else {
                    *visibility = Visibility::Inherited;
                }
            }
        }
    }
}
