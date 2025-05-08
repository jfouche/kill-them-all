use super::Hud;
use crate::{
    components::{
        inventory::{PlayerEquipmentChanged, ToggleInventory},
        player::PlayerAction,
        skills::SkillBookLocation,
    },
    schedule::GameState,
    ui::button::ButtonColors,
};
use bevy::prelude::*;

pub struct HudSkillsPlugin;

impl Plugin for HudSkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_hud_skills)
            .add_observer(toggle_inventory);
    }
}

#[derive(Component)]
#[require(
    Hud,
    Name::new("HUD - Skills"),
    Node {
        position_type: PositionType::Absolute,
        align_items: AlignItems::Center,
        right: Val::Px(50.),
        bottom: Val::Px(15.),
        height: Val::Px(48.),
        ..Default::default()
    }
)]
struct HudSkillsPanel;

#[derive(Component)]
#[require(
    Button,
    ButtonColors {
        normal: Color::srgb_u8(223, 15, 15),
        hovered: Color::srgb_u8(160, 12, 12),
        pressed: Color::srgb_u8(214, 90, 90)
    },
    Node {
        width: Val::Px(50.0),
        height: Val::Percent(100.),
        margin: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    },
    BorderColor(Srgba::BLACK.into())
)]
struct InventoryButton;

fn toggle_inventory(
    trigger: Trigger<Pointer<Click>>,
    mut command: Commands,
    btns: Query<(), With<InventoryButton>>,
) {
    if btns.contains(trigger.target()) {
        command.trigger(ToggleInventory);
    }
}

fn spawn_hud_skills(mut commands: Commands) {
    commands.spawn((
        HudSkillsPanel,
        children![
            (InventoryButton, children![Text::new("I"),]),
            (PlayerAction::Skill1, SkillBookLocation),
            (PlayerAction::Skill2, SkillBookLocation),
            (PlayerAction::Skill3, SkillBookLocation),
            (PlayerAction::Skill4, SkillBookLocation)
        ],
    ));

    // to force to update skills
    commands.trigger(PlayerEquipmentChanged);
}
