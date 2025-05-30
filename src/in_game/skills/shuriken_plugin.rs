use crate::{
    components::{
        affix::PierceChance,
        character::{Character, Target},
        damage::{Damager, DamagerParams, HitDamageRange, ProjectileParams},
        despawn_all,
        item::update_item_info,
        skills::{
            shuriken::{Shuriken, ShurikenAssets, ShurikenLauncher, ShurikenLauncherBook},
            ActivateSkill,
        },
        world_map::LAYER_DAMAGER,
    },
    schedule::GameState,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

const SHURIKEN_SPEED: f32 = 100.0;

pub struct ShurikenPlugin;

impl Plugin for ShurikenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShurikenAssets>()
            .add_systems(OnExit(GameState::InGame), despawn_all::<Shuriken>)
            .add_observer(update_item_info::<ShurikenLauncherBook>())
            .add_observer(launch_shuriken);
    }
}

fn launch_shuriken(
    trigger: Trigger<ActivateSkill>,
    mut commands: Commands,
    skills: Query<(&HitDamageRange, &ChildOf), With<ShurikenLauncher>>,
    characters: Query<(&Transform, &PierceChance, &Target), With<Character>>,
    asset: Res<ShurikenAssets>,
) {
    let (skill_entity, target_pos) = (trigger.0, trigger.1);
    if let Ok((damage_range, child_of)) = skills.get(skill_entity) {
        if let Ok((origin, pierce_chance, target)) = characters.get(child_of.parent()) {
            let origin = origin.translation.xy();
            let velocity = (target_pos - origin).normalize() * SHURIKEN_SPEED;
            commands.spawn((
                Shuriken,
                *damage_range,
                DamagerParams {
                    transform: Transform::from_translation(origin.extend(LAYER_DAMAGER)),
                    collision_groups: Damager::collision_groups(*target),
                },
                ProjectileParams {
                    pierce_chance: *pierce_chance,
                    velocity: Velocity {
                        linvel: velocity,
                        angvel: 2. * PI,
                    },
                },
                Sprite::from_image(asset.shuriken.clone()),
            ));
        }
    }
}
