use std::f32::consts::PI;

use crate::{components::*, in_game::GameRunningSet};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
struct ShurikenAssets {
    shuriken: Handle<Image>,
}

#[derive(Component)]
pub struct ShurikenLauncher {
    dir: Dir2,
}

const BASE_ATTACK_SPEED: f32 = 1.5;

pub fn shuriken_launcher() -> impl Bundle {
    (
        ShurikenLauncher { dir: Dir2::NORTH },
        Name::new("ShurikenLauncher"),
        WeaponBundle::new(DamageRange(2. ..=4.), BASE_ATTACK_SPEED),
    )
}

#[derive(Component)]
pub struct Shuriken;

#[derive(Bundle)]
struct ShurikenBundle {
    tag: Shuriken,
    name: Name,
    ammo: AmmoBundle,
    sprite: SpriteBundle,
}

impl Default for ShurikenBundle {
    fn default() -> Self {
        ShurikenBundle {
            tag: Shuriken,
            name: Name::new("Shuriken"),
            ammo: AmmoBundle::default(),
            sprite: SpriteBundle::default(),
        }
    }
}

const SHURIKEN_SPEED: f32 = 100.0;

pub struct ShurikenPlugin;

impl Plugin for ShurikenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_shuriken_assets).add_systems(
            Update,
            (set_shurikne_direction, launch_shuriken)
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

fn set_shurikne_direction(
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
    characters: Query<(&Transform, &PierceChance)>,
    weapons: Query<(&AttackTimer, &DamageRange, &Parent, &ShurikenLauncher)>,
    asset: Res<ShurikenAssets>,
) {
    let mut rng = rand::thread_rng();
    for (timer, damage_range, parent, lancher) in &weapons {
        if timer.just_finished() {
            if let Ok((transform, pierce_chance)) = characters.get(**parent) {
                let velocity = Velocity {
                    linvel: *lancher.dir * SHURIKEN_SPEED,
                    angvel: 2. * PI,
                };
                let ammo_config = AmmoConfig {
                    damage: damage_range.gen(&mut rng),
                    pierce: *pierce_chance,
                    collider: Collider::ball(8.),
                    velocity,
                };
                commands.spawn(ShurikenBundle {
                    ammo: AmmoBundle::new(ammo_config),
                    sprite: SpriteBundle {
                        transform: *transform,
                        texture: asset.shuriken.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        }
    }
}
