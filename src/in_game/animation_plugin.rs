use crate::components::animation::{AnimationTimer, CyclicAnimation, OneShotAnimation};
use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (cyclic_animation, one_shot_animation));
    }
}

fn cyclic_animation(
    mut query: Query<(&mut CyclicAnimation, &mut AnimationTimer, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut animation, mut timer, mut sprite) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animation.next();
            }
        }
    }
}

fn one_shot_animation(
    mut query: Query<(&mut OneShotAnimation, &mut AnimationTimer, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut animation, mut timer, mut sprite) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if let Some(index) = animation.next() {
                    atlas.index = index;
                }
            }
        }
    }
}
