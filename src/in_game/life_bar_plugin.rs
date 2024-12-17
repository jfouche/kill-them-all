use super::{GameRunningSet, GameState};
use crate::{
    components::{despawn_all, Character, Life, MaxLife},
    ui::{ProgressBar, ProgressBarColor},
};
use bevy::prelude::*;
use std::collections::HashMap;

const BAR_WIDTH: f32 = 30.;
const BAR_HEIGHT: f32 = 8.;

#[derive(Component)]
#[require(
    Name(|| Name::new("LifeBar")),
    Node(|| Node {
        position_type: PositionType::Absolute,
        width: Val::Px(BAR_WIDTH),
        height: Val::Px(BAR_HEIGHT),
        border: UiRect::all(Val::Px(1.)),
        ..Default::default()
    }),
    BackgroundColor(|| BackgroundColor(Color::srgb(1., 0.1, 0.1))),
    BorderColor(|| BorderColor(Color::srgb(1., 0.1, 0.1))),
    ProgressBar,
    ProgressBarColor(|| ProgressBarColor(Color::srgb(1., 0., 0.)))
)]
struct LifeBar;

#[derive(Resource, Default, Deref, DerefMut)]
struct LifeBarMap(HashMap<Entity, Entity>);

pub struct LifeBarPlugin;

impl Plugin for LifeBarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LifeBarMap>()
            .add_systems(OnExit(GameState::InGame), despawn_all::<LifeBar>)
            .add_systems(
                Update,
                (update_life_bar,).in_set(GameRunningSet::EntityUpdate),
            )
            .add_observer(spawn_life_bar);
    }
}

fn spawn_life_bar(
    trigger: Trigger<OnAdd, Character>,
    mut commands: Commands,
    mut life_bar_map: ResMut<LifeBarMap>,
) {
    let life_bar = commands.spawn(LifeBar).id();
    life_bar_map.insert(trigger.entity(), life_bar);
    // TODO: remove when died
}

fn update_life_bar(
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    characters: Query<(Entity, &Transform, &Life, &MaxLife), With<Character>>,
    mut life_bars: Query<(&mut Node, &mut ProgressBar), With<LifeBar>>,
    life_bar_map: Res<LifeBarMap>,
) {
    let (camera, camera_transform) = cameras.get_single().expect("Single camera2d");
    for (entity, transform, life, max_life) in &characters {
        if let Some(life_bar_entity) = life_bar_map.get(&entity) {
            if let Ok((mut node, mut progress)) = life_bars.get_mut(*life_bar_entity) {
                if let Ok(pos) = camera.world_to_viewport(camera_transform, transform.translation) {
                    node.left = Val::Px(pos.x - BAR_WIDTH / 2.);
                    node.top = Val::Px(pos.y - 32.);
                    progress.max = **max_life;
                    progress.value = **life;
                }
            }
        }
    }
}
