use bevy::color::palettes::css::{BLUE, DARK_GRAY, GRAY, ORANGE_RED};

use crate::prelude::*;

pub struct LevelUpMenuPlugin;

impl Plugin for LevelUpMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enter_level_up_state)
            .add_systems(OnEnter(GameState::LevelUp), spawn_level_up_menu)
            .add_systems(OnExit(GameState::LevelUp), despawn_level_up_menu)
            .add_systems(Update, back_to_game.run_if(in_state(GameState::LevelUp)))
            .add_systems(
                Update,
                (
                    upgrade_skill::<MaxLifeButton>,
                    // handle_nav_events::<MaxLifeButton>,
                    upgrade_skill::<MovementSpeedButton>,
                    // handle_nav_events::<MovementSpeedButton>,
                    upgrade_skill::<AttackSpeedButton>,
                    // handle_nav_events::<AttackSpeedButton>,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            // .add_systems(Update, (button_system, print_nav_events))
            ;
    }
}

fn enter_level_up_state(
    mut level_up_rcv: EventReader<LevelUpEvent>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in level_up_rcv.read() {
        warn!("enter_level_up_state");
        if **state == GameState::InGame {
            next_state.set(GameState::LevelUp);
        }
    }
}

// fn button_system(
//     mut interaction_query: Query<(&Focusable, &mut BackgroundColor), Changed<Focusable>>,
// ) {
//     for (focusable, mut material) in interaction_query.iter_mut() {
//         if let FocusState::Focused = focusable.state() {
//             *material = ORANGE_RED.into();
//         } else {
//             *material = DARK_GRAY.into();
//         }
//     }
// }

// fn print_nav_events(mut events: EventReader<NavEvent>) {
//     for event in events.read() {
//         println!("{:?}", event);
//     }
// }

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
                width: Val::Percent(50.0),
                height: Val::Percent(50.),
                align_self: AlignSelf::Center,
                left: Val::Percent(25.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: BLUE.into(),
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

fn back_to_game(mut state: ResMut<NextState<GameState>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        state.set(GameState::InGame);
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
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: GRAY.into(),
            ..default()
        })
        // .insert(Focusable::default())
        .with_children(|btn| {
            btn.spawn(TextBundle::from_sections([
                TextSection::new(label, text_style.clone()),
                TextSection::from_style(text_style),
            ]))
            .insert(BackgroundColor(Color::NONE));
        });
}

///
/// Upgrade a skill of the player, returning back to game
///
fn upgrade_skill<T: Skill + Component>(
    mut q_btn: Query<&Interaction, (Changed<Interaction>, With<T>)>,
    mut q_player: Query<&mut T::SkillComponent, With<Player>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut skill) = q_player.get_single_mut() {
        for interaction in &mut q_btn {
            if *interaction == Interaction::Pressed {
                T::upgrade(&mut skill);
                state.set(GameState::InGame);
            }
        }
    }
}

// fn handle_nav_events<T: Skill + Component>(
//     q_btn: Query<(), With<T>>,
//     mut events: EventReader<NavEvent>,
//     mut q_player: Query<&mut T::SkillComponent, With<Player>>,
//     mut state: ResMut<State<GameState>>,
// ) {
//     if let Ok(mut skill) = q_player.get_single_mut() {
//         for _ in events.nav_iter().activated_in_query(&q_btn) {
//             T::upgrade(&mut skill);
//             state.set(GameState::InGame).unwrap();
//         }
//     }
// }
