use bevy::prelude::*;
use super::Hud;

pub struct HudSkillsPlugin;

impl Plugin for HudSkillsPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
#[require(
    Hud, 
    Name(|| Name::new("HudSkills")),
    Node(|| Node {
        position_type: PositionType::Absolute,
        left: Val::Px(50.),
        top: Val::Px(20.),
        width: Val::Px(300.),
        height: Val::Px(20.),
        border: UiRect::all(Val::Px(2.)),
        ..Default::default()
    })
)]
struct HudSkills;
