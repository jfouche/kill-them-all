use super::inventory_panel::InventoryPanel;
use crate::components::*;
use crate::in_game::back_to_game;
use crate::schedule::*;
use crate::ui::*;
use bevy::prelude::*;
use std::fmt::Display;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::Pause), spawn_pause_menu)
            .add_systems(OnExit(InGameState::Pause), despawn_all::<PauseMenu>)
            .add_systems(
                Update,
                (
                    back_to_game,
                    update_skill::<ArmourText>,
                    update_skill::<LifeText>,
                    update_skill::<LifeRegenText>,
                    update_skill::<MovementSpeedText>,
                    update_skill::<AttackSpeedText>,
                    update_skill::<PierceChanceText>,
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
struct ArmourText;

impl Skill for ArmourText {
    type SkillComponent = Armour;
}

#[derive(Component)]
struct LifeText;

impl Skill for LifeText {
    type SkillComponent = Life;
}

#[derive(Component)]
struct LifeRegenText;

impl Skill for LifeRegenText {
    type SkillComponent = LifeRegen;
}

#[derive(Component)]
struct MovementSpeedText;

impl Skill for MovementSpeedText {
    type SkillComponent = MovementSpeed;
}

#[derive(Component)]
struct AttackSpeedText;

impl Skill for AttackSpeedText {
    type SkillComponent = IncreaseAttackSpeed;
}
#[derive(Component)]
struct PierceChanceText;

impl Skill for PierceChanceText {
    type SkillComponent = PierceChance;
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

fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn_popup("Pause", (PauseMenu, Name::new("PauseMenu")))
        .with_children(|popup| {
            popup
                .spawn(NodeBundle {
                    style: Style {
                        display: bevy::ui::Display::Flex,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(95.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|flex| {
                    flex.spawn(InventoryPanel);
                    spawn_skill(flex, "Armour :", ArmourText);
                    spawn_skill(flex, "Life :", LifeText);
                    spawn_skill(flex, "Life regen :", LifeRegenText);
                    spawn_skill(flex, "Movement speed :", MovementSpeedText);
                    spawn_skill(flex, "Attack speed :", AttackSpeedText);
                    spawn_skill(flex, "Pierce chance :", PierceChanceText);
                    spawn_skill(flex, "Weapon :", WeaponText);
                    spawn_skill(flex, "Experience :", ExperienceText);
                    spawn_skill(flex, "Money :", MoneyText);
                });
        });
}

fn spawn_skill(panel: &mut ChildBuilder, label: impl Into<String>, component: impl Bundle) {
    const MARGIN: Val = Val::Px(12.);
    let text_style = TextStyle {
        font_size: 12.0,
        color: Color::WHITE,
        ..Default::default()
    };
    panel
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(2.0)),
                column_gap: MARGIN,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|row| {
            // Label
            row.spawn(
                TextBundle::from_section(label, text_style.clone())
                    .with_text_justify(JustifyText::Right)
                    .with_style(Style {
                        width: Val::Percent(50.0),
                        ..default()
                    }),
            );
            // Value
            row.spawn((
                component,
                TextBundle::from_section("", text_style).with_style(Style {
                    width: Val::Percent(50.0),

                    ..default()
                }),
            ));
        });
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
            text.sections[0].value = T::format(component);
        }
    }
}
