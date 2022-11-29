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
            )
            .add_system_set(SystemSet::on_update(GameState::GamePaused).with_system(update_speed));
    }
}

#[derive(Component)]
struct PauseMenu;

#[derive(Component)]
struct SpeedText;

fn switch_game_state(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.current() {
            GameState::InGame => state.set(GameState::GamePaused).unwrap(),
            GameState::GamePaused => state.set(GameState::InGame).unwrap(),
        }
    }
}

fn spawn_pause_menu(mut commands: Commands, font: Res<UiFont>) {
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
        })
        .with_children(|menu| {
            spawn_title(menu, font.clone());
            // SKILLS
            spawn_skill(menu, font.clone(), "Speed :", SpeedText);
        });
}

fn spawn_title(menu: &mut ChildBuilder, font: Handle<Font>) {
    menu.spawn(
        TextBundle::from_section(
            "Pause",
            TextStyle {
                font,
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::Center,
            ..Default::default()
        }),
    );
}

fn spawn_skill(
    menu: &mut ChildBuilder,
    font: Handle<Font>,
    label: impl Into<String>,
    component: impl Bundle,
) {
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::WHITE,
    };
    menu.spawn(component).insert(TextBundle::from_sections([
        TextSection::new(label, text_style.clone()),
        TextSection::from_style(text_style),
    ]));
}

fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_speed(
    q_player: Query<&Speed, With<Player>>,
    mut q_text: Query<&mut Text, With<SpeedText>>,
) {
    if let Ok(mut text) = q_text.get_single_mut() {
        if let Ok(speed) = q_player.get_single() {
            text.sections[1].value = format!("{}", speed.0);
        }
    }
}
