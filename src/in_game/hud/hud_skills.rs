use super::Hud;
use crate::{
    components::{
        inventory::{PlayerEquipmentChanged, ToggleInventory},
        player::PlayerAction,
        skills::SkillBookLocation,
    },
    schedule::GameState,
    theme::{interaction::InteractionPalette, widget::button_base},
};
use bevy::prelude::*;

pub struct HudSkillsPlugin;

impl Plugin for HudSkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_hud_skills);
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

fn button_inventory() -> impl Bundle {
    button_base(
        "I",
        toggle_inventory,
        (
            Node {
                width: Val::Px(50.0),
                height: Val::Percent(100.),
                margin: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            InteractionPalette {
                none: Color::srgb_u8(223, 15, 15),
                hovered: Color::srgb_u8(160, 12, 12),
                pressed: Color::srgb_u8(214, 90, 90),
            },
            BorderColor(Srgba::BLACK.into()),
        ),
    )
}

fn toggle_inventory(_trigger: Trigger<Pointer<Click>>, mut command: Commands) {
    command.trigger(ToggleInventory);
}

fn spawn_hud_skills(mut commands: Commands) {
    commands.spawn((
        HudSkillsPanel,
        children![
            button_inventory(),
            (PlayerAction::Skill1, SkillBookLocation),
            (PlayerAction::Skill2, SkillBookLocation),
            (PlayerAction::Skill3, SkillBookLocation),
            (PlayerAction::Skill4, SkillBookLocation)
        ],
    ));

    // to force to update skills
    commands.trigger(PlayerEquipmentChanged);
}
