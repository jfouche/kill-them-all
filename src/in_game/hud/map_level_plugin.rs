use super::Hud;
use crate::in_game::{GameRunningSet, GameState};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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
    Name(|| Name::new("HUD - MapLevelPanel")),
    Node(|| Node {
        position_type: PositionType::Absolute,
        right: Val::Px(0.),
        top: Val::Px(100.),
        padding: UiRect::all(Val::Px(4.)),
        ..Default::default()
    }),
    BackgroundColor(|| BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.5))),
)]
struct MapLevelPanel;

#[derive(Component)]
#[require(
    Text,
    TextFont(|| TextFont::from_font_size(10.)),
    TextColor(|| TextColor(Color::WHITE))
)]
struct MapLevelText;

fn spawn_panel(mut commands: Commands) {
    commands.spawn(MapLevelPanel).with_child(MapLevelText);
}

fn update_panel(
    levels: Query<(&LevelIid, &Name)>,
    mut texts: Query<&mut Text, With<MapLevelText>>,
    current_level: Res<LevelSelection>,
) {
    if let LevelSelection::Iid(ref iid) = *current_level {
        if let Some(name) = levels
            .iter()
            .find(|(liid, _name)| iid.as_str() == liid.as_str())
            .map(|(_liid, name)| name)
        {
            for mut text in &mut texts {
                *text = Text(name.into());
            }
        }
    }
}
