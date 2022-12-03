use crate::prelude::*;
use std::fmt::Display;

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
            .add_system_set(
                SystemSet::on_update(GameState::GamePaused)
                    .with_system(update_skill::<LifeText>)
                    .with_system(update_skill::<MovementSpeedText>)
                    .with_system(update_skill::<AttackSpeedText>)
                    .with_system(update_skill::<WeaponText>)
                    .with_system(update_skill::<MoneyText>)
                    .with_system(update_skill::<ExperienceText>),
            );
    }
}

#[derive(Component)]
struct PauseMenu;

///
/// Trait to print a player skill
///
trait Skill {
    /// Component of the player skill
    type SkillComponent: Component + Display;

    /// Format the skill of the player.
    ///
    /// By default, it formats the skill using the Display trait
    fn format(component: &Self::SkillComponent) -> String {
        format!("{}", component)
    }
}

#[derive(Component)]
struct LifeText;

impl Skill for LifeText {
    type SkillComponent = Life;
}

#[derive(Component)]
struct MovementSpeedText;

impl Skill for MovementSpeedText {
    type SkillComponent = MovementSpeed;
}

#[derive(Component)]
struct AttackSpeedText;

impl Skill for AttackSpeedText {
    type SkillComponent = AttackSpeed;
}

#[derive(Component)]
struct WeaponText;

impl Skill for WeaponText {
    type SkillComponent = Weapon;
}

#[derive(Component)]
struct MoneyText;

impl Skill for MoneyText {
    type SkillComponent = Money;
}

#[derive(Component)]
struct ExperienceText;

impl Skill for ExperienceText {
    type SkillComponent = Experience;
}

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
            spawn_skill(menu, font.clone(), "Life :", LifeText);
            spawn_skill(menu, font.clone(), "Movement speed :", MovementSpeedText);
            spawn_skill(menu, font.clone(), "Attack speed :", AttackSpeedText);
            spawn_skill(menu, font.clone(), "Weapon :", WeaponText);
            spawn_skill(menu, font.clone(), "Money :", MoneyText);
            spawn_skill(menu, font.clone(), "Experience :", ExperienceText);
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

fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
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
    menu.spawn(component)
        .insert(TextBundle::from_sections([
            TextSection::new(label, text_style.clone()),
            TextSection::from_style(text_style),
        ]))
        .insert(BackgroundColor(Color::BLACK));
}

///
/// Update the skill value depending on the Text tag
///
fn update_skill<T: Skill + Component>(
    q_player: Query<&T::SkillComponent, With<Player>>,
    mut q_text: Query<&mut Text, With<T>>,
) {
    if let Ok(mut text) = q_text.get_single_mut() {
        if let Ok(component) = q_player.get_single() {
            text.sections[1].value = T::format(component);
        }
    }
}
