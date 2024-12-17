use crate::components::*;
use bevy::prelude::*;

///
/// A panel that shows the players characteristics
///
#[derive(Component)]
#[require(
    Name(|| Name::new("CharacteristicsPanel")),
    Node(|| Node {
        flex_direction: FlexDirection::Column,
        ..Default::default()
    })
)]
pub struct CharacteristicsPanel;

///
/// Row for a characteristic
///
#[derive(Component)]
#[require(
    Node(|| Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Row,
        padding: UiRect::all(Val::Px(2.0)),
        column_gap: Val::Px(12.),
        ..Default::default()
    })
)]
struct CharacteristicRow;

///
/// label node for a characteristic
///
#[derive(Component)]
#[require(
    Text,
    TextFont(|| TextFont::from_font_size(12.)),
    TextColor(|| TextColor(Color::WHITE)),
    TextLayout(|| TextLayout::new_with_justify(JustifyText::Right)),
    Node(|| Node {
        width: Val::Percent(50.0),
        ..Default::default()
    })
)]
struct CharacteristicLabel;

///
/// Value node for a characteristic
///
#[derive(Component)]
#[require(
    Text,
    TextFont(|| TextFont::from_font_size(12.)),
    TextColor(|| TextColor(Color::WHITE)),
    Node(|| Node {
        width: Val::Percent(50.0),
        ..Default::default()
    })
)]
struct CharacteristicValue;

pub struct CharacteristicsPanelPlugin;

impl Plugin for CharacteristicsPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_caracteristics);
    }
}

fn spawn_caracteristics(
    trigger: Trigger<OnAdd, CharacteristicsPanel>,
    mut commands: Commands,
    players: Query<
        (
            &Armour,
            (&MaxLife, &LifeRegen),
            &IncreaseMovementSpeed,
            &IncreaseAttackSpeed,
            &PierceChance,
            (&MoreDamage, &IncreaseDamage),
        ),
        With<Player>,
    >,
) {
    for (
        armour,
        (max_life, life_regen),
        move_speed,
        incr_attack_speed,
        pierce_chance,
        (more_damage, incr_damage),
    ) in &players
    {
        commands.entity(trigger.entity()).with_children(|panel| {
            spawn_caracteristic(panel, "Armour :", armour);
            spawn_caracteristic(panel, "Maximum life :", max_life);
            spawn_caracteristic(panel, "Life regeneration :", life_regen);
            spawn_caracteristic(panel, "Movement speed :", move_speed);
            spawn_caracteristic(panel, "Attack speed :", incr_attack_speed);
            spawn_caracteristic(panel, "Pierce chance :", pierce_chance);
            spawn_caracteristic(panel, "More damage :", more_damage);
            spawn_caracteristic(panel, "Increase damage :", incr_damage);
        });
    }
}

fn spawn_caracteristic<T: std::fmt::Display>(commands: &mut ChildBuilder, label: &str, value: &T) {
    commands.spawn(CharacteristicRow).with_children(|row| {
        row.spawn((CharacteristicLabel, Text(label.into())));
        row.spawn((CharacteristicValue, Text(value.to_string())));
    });
}
