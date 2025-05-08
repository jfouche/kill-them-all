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

fn spawn_hud_skills(mut commands: Commands) {
    commands.spawn(HudSkillsPanel).with_children(|p| {
        p.spawn(InventoryButton)
            .with_children(|b| {
                b.spawn(Text::new("I"));
            })
            .observe(|_: Trigger<Pointer<Click>>, mut command: Commands| {
                command.trigger(ToggleInventory);
            });

        let entity = p.spawn((PlayerAction::Skill1, SkillBookLocation)).id();
        let entity = p.spawn((PlayerAction::Skill2, SkillBookLocation)).id();
        let entity = p.spawn((PlayerAction::Skill3, SkillBookLocation)).id();
        let entity = p.spawn((PlayerAction::Skill4, SkillBookLocation)).id();
    });

    // to force to update skills
    commands.trigger(PlayerEquipmentChanged);
}
