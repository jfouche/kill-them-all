mod death_aura_plugin;
mod fireball_plugin;
mod mine_plugin;
mod shuriken_plugin;

use super::GameRunningSet;
use crate::components::*;
use bevy::prelude::*;

pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            fireball_plugin::FireballPlugin,
            shuriken_plugin::ShurikenPlugin,
            mine_plugin::MinePlugin,
            death_aura_plugin::DeathAuraPlugin,
        ))
        .init_resource::<SkillAssets>()
        .add_observer(update_character_observers)
        .add_systems(
            Update,
            update_skills_affected_by_aoe.in_set(GameRunningSet::EntityUpdate),
        );
    }
}

fn update_character_observers(trigger: Trigger<OnAdd, Character>, mut commands: Commands) {
    commands.entity(trigger.entity()).observe(try_pierce);
}

fn try_pierce(
    trigger: Trigger<HitEvent>,
    mut commands: Commands,
    mut projectiles: Query<&mut PierceChance, With<Projectile>>,
) {
    if let Ok(mut pierce_chance) = projectiles.get_mut(trigger.entity()) {
        let mut rng = rand::thread_rng();
        if !pierce_chance.try_pierce(&mut rng) {
            // Didn't pierce => despawn projectile
            commands.entity(trigger.entity()).despawn();
        }
    }
}

fn update_skills_affected_by_aoe(
    mut skills: Query<(&mut Transform, &Parent), (With<Skill>, With<AffectedByAreaOfEffect>)>,
    characters: Query<&IncreaseAreaOfEffect, With<Character>>,
) {
    for (mut transform, parent) in &mut skills {
        if let Ok(incr) = characters.get(**parent) {
            let scale = 1. + **incr / 100.;
            transform.scale = Vec3::splat(scale);
        }
    }
}