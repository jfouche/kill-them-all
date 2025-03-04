use bevy::prelude::*;

#[derive(Default, Deref, DerefMut)]
pub struct MultipleObservers(pub Vec<Observer>);

impl MultipleObservers {
    pub fn watch_entity(&mut self, entity: Entity) {
        for o in self.0.iter_mut() {
            o.watch_entity(entity);
        }
    }
}
