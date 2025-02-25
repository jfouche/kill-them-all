use super::{GameRunningSet, GameState};
use crate::{
    camera::MainCamera,
    components::{
        character::{CharacterDyingEvent, Life, MaxLife},
        monster::Monster,
    },
    ui::progressbar::{ProgressBar, ProgressBarColor},
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
    ZIndex(|| ZIndex(-1)),
    BackgroundColor(|| BackgroundColor(Color::BLACK)),
    BorderColor(|| BorderColor(Color::BLACK)),
    ProgressBar,
    ProgressBarColor(|| ProgressBarColor(Color::srgb(1., 0., 0.)))
)]
pub struct LifeBar;

/// Stores a map with the [Monster] entity as key, and the [LifeBar] entity as value
#[derive(Resource, Default, Deref, DerefMut)]
struct LifeBarMap(HashMap<Entity, Entity>);

/// Plugin that show a life bar over the [Monster]s
pub struct LifeBarPlugin;

impl Plugin for LifeBarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LifeBarMap>()
            .add_systems(OnExit(GameState::InGame), clear_life_bars)
            .add_systems(
                Update,
                (update_life_bar,).in_set(GameRunningSet::EntityUpdate),
            )
            .add_observer(spawn_life_bar);
    }
}

fn spawn_life_bar(
    trigger: Trigger<OnAdd, Monster>,
    mut commands: Commands,
    mut life_bar_map: ResMut<LifeBarMap>,
) {
    let life_bar = commands.spawn(LifeBar).id();
    life_bar_map.insert(trigger.entity(), life_bar);

    commands
        .entity(trigger.entity())
        .observe(remove_life_bar_on_monster_dying);
}

fn remove_life_bar_on_monster_dying(
    trigger: Trigger<CharacterDyingEvent>,
    mut commands: Commands,
    mut life_bar_map: ResMut<LifeBarMap>,
) {
    if let Some(bar) = life_bar_map.remove(&trigger.entity()) {
        commands.entity(bar).despawn_recursive();
    }
}

fn clear_life_bars(mut commands: Commands, mut life_bar_map: ResMut<LifeBarMap>) {
    for life_bar in life_bar_map.values() {
        commands.entity(*life_bar).despawn_recursive();
    }
    life_bar_map.clear();
}

fn update_life_bar(
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    monsters: Query<(Entity, &Transform, &Life, &MaxLife), With<Monster>>,
    mut life_bars: Query<(&mut Node, &mut ProgressBar), With<LifeBar>>,
    life_bar_map: Res<LifeBarMap>,
) {
    let (camera, camera_transform) = cameras.get_single().expect("Single MainCamera");
    for (entity, transform, life, max_life) in &monsters {
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
