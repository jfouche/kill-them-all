use super::{Ammo, AttackTimer, BaseAttackSpeed, Character, DamageRange, Weapon};
use crate::in_game::GameRunningSet;
use bevy::prelude::*;

///
/// Weapon that drop a mine regularly
///
#[derive(Component)]
#[require(
    Name(||Name::new("MineDropper")),
    Weapon,
    DamageRange(|| DamageRange::new(1., 5.)),
    BaseAttackSpeed(|| BaseAttackSpeed(0.7))
)]
pub struct MineDropper;

///
/// Mine
///
#[derive(Component)]
#[require(
    Name(|| Name::new("Mine")),
    Ammo,
    MineExplodeTimer
)]
pub struct Mine;

#[derive(Component, Deref, DerefMut, Reflect)]
struct MineExplodeTimer(Timer);

impl Default for MineExplodeTimer {
    fn default() -> Self {
        MineExplodeTimer(Timer::from_seconds(1.5, TimerMode::Once))
    }
}

#[derive(Resource)]
struct MineAssets {
    mesh: Handle<Mesh>,
    color: Handle<ColorMaterial>,
}

pub struct MinePlugin;

impl Plugin for MinePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, load_assets).add_systems(
            Update,
            (drop_mine, mine_explosion).in_set(GameRunningSet::EntityUpdate),
        );
    }
}

fn load_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(MineAssets {
        mesh: meshes.add(Circle::new(6.)),
        color: materials.add(Color::srgb(0.5, 0.5, 0.5)),
    });
}

fn drop_mine(
    mut commands: Commands,
    mut mine_droppers: Query<(&AttackTimer, &DamageRange, &Parent), With<MineDropper>>,
    characters: Query<&Transform, With<Character>>,
    assets: Res<MineAssets>,
) {
    for (timer, damage_range, parent) in &mut mine_droppers {
        if timer.just_finished() {
            if let Ok(Transform { translation, .. }) = characters.get(**parent) {
                info!("drop_mine() : {}", translation);
                // drop mine at character position
                let mut rng = rand::thread_rng();
                let damage = damage_range.gen(&mut rng);
                commands.spawn((
                    Mine,
                    damage,
                    Transform::from_xyz(translation.x, translation.y, 12.),
                    Mesh2d(assets.mesh.clone()),
                    MeshMaterial2d(assets.color.clone()),
                ));
            }
        }
    }
}

fn mine_explosion(
    mut commands: Commands,
    mut mines: Query<(Entity, &mut MineExplodeTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in &mut mines {
        timer.tick(time.delta());
        if timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
