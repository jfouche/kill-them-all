use bevy::prelude::*;

/// Helper trait to manage `Vec<Observer>` that watch multiple entities
pub trait VecObserversExt {
    fn with_observers(self, observers: Vec<Observer>) -> Self;
    fn watch_entity(&mut self, entity: Entity);
}

impl VecObserversExt for Vec<Observer> {
    fn watch_entity(&mut self, entity: Entity) {
        self.iter_mut().for_each(|o| o.watch_entity(entity));
    }

    fn with_observers(mut self, observers: Vec<Observer>) -> Self {
        for o in observers {
            self.push(o);
        }
        self
    }
}
