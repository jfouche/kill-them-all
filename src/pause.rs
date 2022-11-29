use crate::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(switch_game_state)
            .add_system_set(
                SystemSet::on_enter(GameState::GamePaused).with_system(spawn_pause_menu),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::GamePaused).with_system(despawn_pause_menu),
            );
    }
}

#[derive(Component)]
struct PauseMenu;

fn switch_game_state(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.current() {
            GameState::InGame => state.set(GameState::GamePaused).unwrap(),
            GameState::GamePaused => state.set(GameState::InGame).unwrap(),
        }
    }
}

fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn(PauseMenu)
        .insert(Name::new("Pause menu"))
        .insert(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(50.0), Val::Percent(50.)),
                align_self: AlignSelf::Center,
                position: UiRect::left(Val::Percent(25.)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: Color::BLUE.into(),
            ..Default::default()
        });
}

fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
