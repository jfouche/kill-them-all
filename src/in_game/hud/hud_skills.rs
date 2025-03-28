use super::Hud;
use crate::{
    components::{
        inventory::{PlayerEquipmentChanged, ToggleInventory},
        player::PlayerAction,
        skills::SkillGemLocation,
    }, in_game::menu::popup_info::SpawnInfoPopupObservers, schedule::GameState, ui::button::ButtonColors, utils::observers::VecObserversExt
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
    Name(|| Name::new("HUD - Skills")),
    Node(|| Node {
        position_type: PositionType::Absolute,
        align_items: AlignItems::Center,
        right: Val::Px(50.),
        bottom: Val::Px(15.),
        height: Val::Px(48.),
        ..Default::default()
    })
)]
struct HudSkillsPanel;

#[derive(Component)]
#[require(
    Button,
    ButtonColors(|| ButtonColors {
        normal: Color::srgb_u8(223, 15, 15), 
        hovered: Color::srgb_u8(160, 12, 12), 
        pressed: Color::srgb_u8(214, 90, 90)
    }),
    Node(|| Node {
        width: Val::Px(50.0),
        height: Val::Percent(100.),
        margin: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    }),
    BorderColor(|| BorderColor(Srgba::BLACK.into()))
)]
struct InventoryButton;

fn spawn_hud_skills(mut commands: Commands) {
    let mut observers = vec![].with_observers(SpawnInfoPopupObservers::observers());
    commands.spawn(HudSkillsPanel).with_children(|p| {
        p.spawn(InventoryButton)
            .with_children(|b| {
                b.spawn(Text::new("I"));
            })
            .observe(|_: Trigger<Pointer<Click>>, mut command: Commands| {
                command.trigger(ToggleInventory);
            });

        let entity = p.spawn((PlayerAction::Skill1, SkillGemLocation)).id();
        observers.watch_entity(entity);
        let entity = p.spawn((PlayerAction::Skill2, SkillGemLocation)).id();
        observers.watch_entity(entity);
        let entity = p.spawn((PlayerAction::Skill3, SkillGemLocation)).id();
        observers.watch_entity(entity);
        let entity = p.spawn((PlayerAction::Skill4, SkillGemLocation)).id();
        observers.watch_entity(entity);
    });
    commands.spawn_batch(observers);

    // to force to update skills
    commands.trigger(PlayerEquipmentChanged);
}
