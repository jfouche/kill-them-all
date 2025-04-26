use super::Hud;
use crate::{
    components::world_map::CurrentMapLevel,
    schedule::{GameRunningSet, GameState},
};
use bevy::prelude::*;

pub struct MapLevelPlugin;

impl Plugin for MapLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_panel)
            .add_systems(Update, update_panel.in_set(GameRunningSet::EntityUpdate));
    }
}

#[derive(Component)]
#[require(
    Hud,
    Name::new("HUD - MapLevelPanel"),
    Node {
        position_type: PositionType::Absolute,
        right: Val::Px(0.),
        top: Val::Px(100.),
        padding: UiRect::all(Val::Px(4.)),
        ..Default::default()
    },
    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.5)),
)]
struct MapLevelPanel;

#[derive(Component)]
#[require(
    Text,
    TextFont = TextFont::from_font_size(10.),
    TextColor(Color::WHITE)
)]
struct MapLevelText;

fn spawn_panel(mut commands: Commands) {
    commands.spawn(MapLevelPanel).with_child(MapLevelText);
}

fn update_panel(
    mut texts: Query<&mut Text, With<MapLevelText>>,
    current_map_level: Res<CurrentMapLevel>,
) {
    for mut text in &mut texts {
        let info = format!(
            "{}\nmonster_level: {}",
            current_map_level.name, current_map_level.monster_level
        );
        *text = Text(info);
    }
}
