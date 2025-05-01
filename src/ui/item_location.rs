use crate::components::{
    inventory::PlayerEquipmentChanged,
    item::ItemEntity,
    player::{Player, PlayerAction, PlayerSkills},
    skills::SkillBookLocation,
};
use bevy::prelude::*;

pub struct ItemLocationPlugin;

impl Plugin for ItemLocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(update_skills_location);
    }
}

fn update_skills_location(
    _trigger: Trigger<PlayerEquipmentChanged>,
    skills: Query<&PlayerSkills, With<Player>>,
    mut locations: Query<(&mut ItemEntity, &PlayerAction), With<SkillBookLocation>>,
) {
    if let Ok(skills) = skills.single() {
        for (mut item_entity, action) in &mut locations {
            item_entity.0 = skills.get(*action);
        }
    }
}
