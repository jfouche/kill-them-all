use crate::components::*;
use crate::resources::UiFont;
use crate::schedule::*;
use crate::ui::spawn_popup;
use bevy::prelude::*;
use std::fmt::Display;

use super::back_to_game;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::Pause), spawn_pause_menu)
            .add_systems(OnExit(InGameState::Pause), despawn_all::<PauseMenu>)
            .add_systems(
                Update,
                (
                    back_to_game,
                    update_skill::<LifeText>,
                    update_skill::<MovementSpeedText>,
                    update_skill::<AttackSpeedText>,
                    update_skill::<WeaponText>,
                    update_skill::<MoneyText>,
                    update_skill::<ExperienceText>,
                )
                    .run_if(in_state(InGameState::Pause)),
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

fn spawn_pause_menu(commands: Commands, font: Res<UiFont>) {
    spawn_popup(commands, "Pause", PauseMenu, |popup| {
        spawn_skill(popup, font.clone(), "Life :", LifeText);
        spawn_skill(popup, font.clone(), "Movement speed :", MovementSpeedText);
        spawn_skill(popup, font.clone(), "Attack speed :", AttackSpeedText);
        spawn_skill(popup, font.clone(), "Weapon :", WeaponText);
        spawn_skill(popup, font.clone(), "Money :", MoneyText);
        spawn_skill(popup, font.clone(), "Experience :", ExperienceText);
    });
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
        .insert(
            TextBundle::from_sections([
                TextSection::new(label, text_style.clone()),
                TextSection::from_style(text_style),
            ])
            .with_style(Style {
                width: Val::Percent(90.0),
                height: Val::Auto,
                margin: UiRect::all(Val::Px(5.)),
                padding: UiRect::all(Val::Px(5.)),
                ..default()
            }),
        )
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
