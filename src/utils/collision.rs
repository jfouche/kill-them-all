use bevy::{
    ecs::query::{QueryData, QueryFilter, WorldQuery},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

/// Filter CollisionEvent::Started events
pub fn start_event_filter(event: &CollisionEvent) -> Option<(&Entity, &Entity)> {
    match event {
        CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
        _ => None,
    }
}

/// QueryEither
///
/// Example:
/// ```
/// query1.iter().filter_map(query.get_either(e1, e2)).map(|(data, e1, e2)|{})
/// ```
pub trait QueryEither<'w, D>
where
    D: QueryData<ReadOnly = D>,
{
    /// get either `e1` or `e2`, returning a `([QueryData], [Entity from query], [other Entity])`
    fn get_either(
        &'w self,
        e1: Entity,
        e2: Entity,
    ) -> Option<(<D as WorldQuery>::Item<'w>, Entity, Entity)>;
}

impl<'w, D, F> QueryEither<'w, D> for Query<'w, '_, D, F>
where
    D: QueryData<ReadOnly = D>,
    F: QueryFilter,
{
    fn get_either(
        &'w self,
        e1: Entity,
        e2: Entity,
    ) -> Option<(<D as WorldQuery>::Item<'w>, Entity, Entity)> {
        self.get(e1)
            .map(|data| (data, e1, e2))
            .or(self.get(e2).map(|data| (data, e2, e1)))
            .ok()
    }
}

// /// The [EqEither] trait allow to check if self is equal to either
// /// one value or another
// pub trait EqEither {
//     fn eq_either(&self, v1: Self, v2: Self) -> bool;
// }

// impl<T> EqEither for T
// where
//     T: Copy + PartialEq,
// {
//     fn eq_either(&self, v1: Self, v2: Self) -> bool {
//         self == &v1 || self == &v2
//     }
// }
