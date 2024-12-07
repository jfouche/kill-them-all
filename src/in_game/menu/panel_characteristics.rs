use crate::components::*;
use bevy::prelude::*;

pub fn characteristics_panel() -> impl Bundle {
    (
        CharacteristicsPanel,
        Name::new("CharacteristicsPanel"),
        Node {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
    )
}

#[derive(Component)]
pub struct CharacteristicsPanel;

///
/// Trait to print a player skill
///
trait Skill {
    /// Component of the player skill
    type SkillComponent: Component + crate::components::Label;

    /// Format the skill of the player.
    ///
    /// By default, it formats the skill using the Display trait
    fn format(component: &Self::SkillComponent) -> String {
        component.label()
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
    type SkillComponent = IncreaseMovementSpeed;
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

// #[derive(Component)]
// struct WeaponText;

// impl Skill for WeaponText {
//     type SkillComponent = Weapon;
// }

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

pub struct CharacteristicsPanelPlugin;

impl Plugin for CharacteristicsPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hooks).add_systems(
            Update,
            (
                update_skill::<ArmourText>,
                update_skill::<LifeText>,
                update_skill::<LifeRegenText>,
                update_skill::<MovementSpeedText>,
                update_skill::<AttackSpeedText>,
                update_skill::<PierceChanceText>,
                // update_skill::<WeaponText>,
                update_skill::<MoneyText>,
                update_skill::<ExperienceText>,
            )
                .run_if(any_with_component::<CharacteristicsPanel>),
        );
    }
}

fn setup_hooks(world: &mut World) {
    world
        .register_component_hooks::<CharacteristicsPanel>()
        .on_add(|mut world, entity, _component_id| {
            info!("on_add::<CharacteristicsPanel>");
            world.commands().entity(entity).with_children(|panel| {
                spawn_skill(panel, "Armour :", ArmourText);
                spawn_skill(panel, "Life :", LifeText);
                spawn_skill(panel, "Life regen :", LifeRegenText);
                spawn_skill(panel, "Movement speed :", MovementSpeedText);
                spawn_skill(panel, "Attack speed :", AttackSpeedText);
                spawn_skill(panel, "Pierce chance :", PierceChanceText);
                // spawn_skill(panel, "Weapon :", WeaponText);
                spawn_skill(panel, "Experience :", ExperienceText);
                spawn_skill(panel, "Money :", MoneyText);
            });
        });
}

fn spawn_skill(panel: &mut ChildBuilder, label: impl Into<String>, component: impl Bundle) {
    const MARGIN: Val = Val::Px(12.);

    let text_font = TextFont::from_font_size(12.);
    let text_color = TextColor(Color::WHITE);

    panel
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(2.0)),
            column_gap: MARGIN,
            ..Default::default()
        })
        .with_children(|row| {
            // Label
            row.spawn((
                Text(label.into()),
                text_font.clone(),
                text_color,
                TextLayout::new_with_justify(JustifyText::Right),
                Node {
                    width: Val::Percent(50.0),
                    ..default()
                },
            ));
            // Value
            row.spawn((
                component,
                Text("".into()),
                text_font,
                text_color,
                Node {
                    width: Val::Percent(50.0),
                    ..default()
                },
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
            text.0 = T::format(component);
        }
    }
}
