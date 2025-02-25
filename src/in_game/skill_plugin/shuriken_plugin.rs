use crate::{
    components::*,
    schedule::{GameRunningSet, GameState},
};
use affix::PierceChance;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use character::{Character, Target};
use damage::{Damager, DamagerParams, HitDamageRange, ProjectileParams};
use equipment::weapon::AttackTimer;
use skills::shuriken::{Shuriken, ShurikenAssets, ShurikenLauncher};
use std::f32::consts::PI;

const SHURIKEN_SPEED: f32 = 100.0;

pub struct ShurikenPlugin;

impl Plugin for ShurikenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShurikenAssets>()
            .add_systems(OnExit(GameState::InGame), despawn_all::<Shuriken>)
            .add_systems(
                Update,
                (set_shuriken_direction, launch_shuriken)
                    .chain()
                    .in_set(GameRunningSet::EntityUpdate),
            );
    }
}

fn set_shuriken_direction(
    characters: Query<&Velocity>,
    mut weapons: Query<(&mut ShurikenLauncher, &Parent)>,
) {
    for (mut launcher, parent) in &mut weapons {
        if let Ok(velocity) = characters.get(**parent) {
            if let Ok(dir) = Dir2::new(velocity.linvel) {
                launcher.dir = dir;
            }
        }
    }
}

fn launch_shuriken(
    mut commands: Commands,
    weapons: Query<(&AttackTimer, &HitDamageRange, &Parent, &ShurikenLauncher)>,
    characters: Query<(&Transform, &PierceChance, &Target), With<Character>>,
    asset: Res<ShurikenAssets>,
) {
    for (timer, damage_range, parent, lancher) in &weapons {
        if timer.just_finished() {
            if let Ok((transform, pierce_chance, target)) = characters.get(**parent) {
                commands.spawn((
                    Shuriken,
                    *damage_range,
                    DamagerParams {
                        transform: *transform,
                        collision_groups: Damager::collision_groups(*target),
                    },
                    ProjectileParams {
                        pierce_chance: *pierce_chance,
                        velocity: Velocity {
                            linvel: *lancher.dir * SHURIKEN_SPEED,
                            angvel: 2. * PI,
                        },
                    },
                    Sprite::from_image(asset.shuriken.clone()),
                ));
            }
        }
    }
}
