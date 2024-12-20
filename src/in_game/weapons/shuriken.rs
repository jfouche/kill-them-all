use crate::{components::*, in_game::GameRunningSet};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

#[derive(Resource)]
struct ShurikenAssets {
    shuriken: Handle<Image>,
}

///
/// Weapon that launch [Shuriken]s
///
#[derive(Component)]
#[require(
    Weapon,
    Name(|| Name::new("ShurikenLauncher")),
    BaseDamageRange(|| BaseDamageRange::new(2., 4.)),
    BaseAttackSpeed(|| BaseAttackSpeed(1.5)),
)]
pub struct ShurikenLauncher {
    dir: Dir2,
}

impl Default for ShurikenLauncher {
    fn default() -> Self {
        ShurikenLauncher { dir: Dir2::NORTH }
    }
}

///
/// A shuriken projectile
///
#[derive(Component)]
#[require(
    Name(|| Name::new("Shuriken")),
    Projectile,
    Sprite,
    Collider(|| Collider::ball(8.))
)]
struct Shuriken;

const SHURIKEN_SPEED: f32 = 100.0;

pub struct ShurikenPlugin;

impl Plugin for ShurikenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_shuriken_assets).add_systems(
            Update,
            (set_shuriken_direction, launch_shuriken)
                .chain()
                .in_set(GameRunningSet::EntityUpdate),
        );
    }
}

fn load_shuriken_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = ShurikenAssets {
        shuriken: asset_server.load("shuriken.png"),
    };
    commands.insert_resource(assets);
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
    weapons: Query<(&AttackTimer, &DamageRange, &Parent, &ShurikenLauncher)>,
    characters: Query<(&Transform, &PierceChance, &Target), With<Character>>,
    asset: Res<ShurikenAssets>,
) {
    for (timer, damage_range, parent, lancher) in &weapons {
        if timer.just_finished() {
            if let Ok((transform, pierce_chance, target)) = characters.get(**parent) {
                commands.spawn((
                    Shuriken,
                    AmmoParams {
                        damage_range: *damage_range,
                        transform: *transform,
                        collision_groups: Ammo::collision_groups(*target),
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
