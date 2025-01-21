use bevy::{picking::pointer::PointerLocation, prelude::*};

pub trait WorldPosition {
    fn world_position(&self, camera: &Camera, camera_transform: &GlobalTransform) -> Option<Vec2>;
}

impl WorldPosition for &PointerLocation {
    fn world_position(&self, camera: &Camera, camera_transform: &GlobalTransform) -> Option<Vec2> {
        self.location
            .as_ref()
            .map(|l| {
                camera
                    .viewport_to_world_2d(camera_transform, l.position)
                    .ok()
            })
            .flatten()
    }
}

pub const MAP_DEPTH: f32 = 100.;
pub const ITEM_DEPTH: f32 = 90.;
