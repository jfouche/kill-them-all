use crate::prelude::*;
use bevy_ui_navigation::prelude::*;

pub struct LevelUpMenuPlugin;

impl Plugin for LevelUpMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enter_level_up_state)
            .add_system_set(
                SystemSet::on_enter(GameState::LevelUp).with_system(spawn_level_up_menu),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::LevelUp).with_system(despawn_level_up_menu),
            )
            .add_system_set(
                SystemSet::on_update(GameState::LevelUp)
                    .with_system(back_to_game)
                    .with_system(upgrade_skill::<MaxLifeButton>)
                    .with_system(upgrade_skill::<MovementSpeedButton>)
                    .with_system(upgrade_skill::<AttackSpeedButton>),
            )
            .add_system(button_system)
            .add_system(print_nav_events);
    }
}

fn enter_level_up_state(
    mut level_up_rcv: EventReader<LevelUpEvent>,
    mut state: ResMut<State<GameState>>,
) {
    for _ in level_up_rcv.iter() {
        warn!("enter_level_up_state");
        if state.current() == &GameState::InGame {
            state.set(GameState::LevelUp).unwrap();
        }
    }
}

fn button_system(
    mut interaction_query: Query<(&Focusable, &mut BackgroundColor), Changed<Focusable>>,
) {
    for (focusable, mut material) in interaction_query.iter_mut() {
        if let FocusState::Focused = focusable.state() {
            *material = Color::ORANGE_RED.into();
        } else {
            *material = Color::DARK_GRAY.into();
        }
    }
}

fn print_nav_events(mut events: EventReader<NavEvent>) {
    for event in events.iter() {
        println!("{:?}", event);
    }
}

#[derive(Component)]
struct LevelUpMenu;

///
/// Trait to print a player skill
///
trait Skill {
    /// Component of the player skill
    type SkillComponent: Component;

    /// upgrade the skill
    fn upgrade(component: &mut Self::SkillComponent);
}

#[derive(Component)]
struct MaxLifeButton;

impl Skill for MaxLifeButton {
    type SkillComponent = Life;
    fn upgrade(component: &mut Self::SkillComponent) {
        component.increases(10.);
    }
}

#[derive(Component)]
struct AttackSpeedButton;

impl Skill for AttackSpeedButton {
    type SkillComponent = AttackSpeed;
    fn upgrade(component: &mut Self::SkillComponent) {
        component.increases(10.);
    }
}

#[derive(Component)]
struct MovementSpeedButton;

impl Skill for MovementSpeedButton {
    type SkillComponent = MovementSpeed;
    fn upgrade(component: &mut Self::SkillComponent) {
        component.increases(10.);
    }
}

#[derive(Component)]
struct DamageButton;

fn spawn_level_up_menu(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn(LevelUpMenu)
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
        .with_children(|window| {
            spawn_title(window, font.clone());
            spawn_skill(window, font.clone(), "Max life", MaxLifeButton);
            spawn_skill(window, font.clone(), "Attack speed", AttackSpeedButton);
            spawn_skill(window, font.clone(), "Movement speed", MovementSpeedButton);
            spawn_skill(window, font.clone(), "Damage", DamageButton);
        });
}

fn despawn_level_up_menu(mut commands: Commands, query: Query<Entity, With<LevelUpMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn back_to_game(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) && state.current() == &GameState::LevelUp {
        state.set(GameState::InGame).unwrap();
    }
}

fn spawn_title(menu: &mut ChildBuilder, font: Handle<Font>) {
    menu.spawn(
        TextBundle::from_section(
            "Level up",
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
        color: Color::BLACK,
    };
    menu.spawn(component)
        .insert(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::GOLD.into(),
            ..default()
        })
        .insert(Focusable::default())
        .with_children(|btn| {
            btn.spawn(TextBundle::from_sections([
                TextSection::new(label, text_style.clone()),
                TextSection::from_style(text_style),
            ]))
            .insert(BackgroundColor(Color::GOLD));
        });
}

///
/// Upgrade a skill of the player, returning back to game
///
fn upgrade_skill<T: Skill + Component>(
    mut q_btn: Query<&Interaction, (Changed<Interaction>, With<T>)>,
    mut q_player: Query<&mut T::SkillComponent, With<Player>>,
    mut state: ResMut<State<GameState>>,
) {
    if let Ok(mut skill) = q_player.get_single_mut() {
        for interaction in &mut q_btn {
            if *interaction == Interaction::Clicked {
                T::upgrade(&mut skill);
                state.set(GameState::InGame).unwrap();
            }
        }
    }
}
