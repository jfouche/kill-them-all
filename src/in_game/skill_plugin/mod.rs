mod death_aura_plugin;
mod fireball_plugin;
mod mine_plugin;
mod shuriken_plugin;

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
        .add_observer(update_character_observers);
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
