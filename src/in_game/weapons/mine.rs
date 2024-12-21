use super::{
    AttackTimer, BaseAttackSpeed, Character, CyclicAnimation, DamageRange, Damager, DamagerParams,
    OneShotAnimation, Target, Weapon,
};
use crate::in_game::GameRunningSet;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups};

///
/// Weapon that drop a mine regularly
///
#[derive(Component)]
#[require(
    Name(||Name::new("MineDropper")),
    Weapon,
    DamageRange(|| DamageRange::new(1., 5.)),
    BaseAttackSpeed(|| BaseAttackSpeed(0.6))
)]
pub struct MineDropper;

///
/// Mine
///
#[derive(Component)]
#[require(
    Name(|| Name::new("Mine")),
    Damager,
    Collider(|| Collider::ball(8.)),
    MineExplodeTimer,
    Sprite,
    CyclicAnimation(|| CyclicAnimation::new(0..2))
)]
struct Mine;

#[derive(Component, Deref, DerefMut, Reflect)]
struct MineExplodeTimer(Timer);

impl Default for MineExplodeTimer {
    fn default() -> Self {
        MineExplodeTimer(Timer::from_seconds(1.5, TimerMode::Once))
    }
}

#[derive(Component)]
#[require(
    Damager,
    Collider(|| Collider::ball(16.)),
    Sprite,
    OneShotAnimation(|| OneShotAnimation::new(0..8))
)]
struct MineExplosion;

#[derive(Resource)]
struct MineAssets {
    mine_texture: Handle<Image>,
    mine_atlas_layout: Handle<TextureAtlasLayout>,
    explosion_texture: Handle<Image>,
    explosion_atlas_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for MineAssets {
    fn from_world(world: &mut World) -> Self {
        let mine_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2, 1, None, None);
        let explosion_atlas_layout =
            TextureAtlasLayout::from_grid(UVec2::new(32, 32), 8, 1, None, None);

        MineAssets {
            mine_texture: world.resource::<AssetServer>().load("mine.png"),
            mine_atlas_layout: world
                .resource_mut::<Assets<TextureAtlasLayout>>()
                .add(mine_atlas_layout),
            explosion_texture: world.resource::<AssetServer>().load("mine_explosion.png"),
            explosion_atlas_layout: world
                .resource_mut::<Assets<TextureAtlasLayout>>()
                .add(explosion_atlas_layout),
        }
    }
}

pub struct MinePlugin;

impl Plugin for MinePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<MineAssets>().add_systems(
            Update,
            (drop_mine, mine_explosion, despawn_explosion).in_set(GameRunningSet::EntityUpdate),
        );
    }
}

fn drop_mine(
    mut commands: Commands,
    mut mine_droppers: Query<(&AttackTimer, &DamageRange, &Parent), With<MineDropper>>,
    characters: Query<(&Transform, &Target), With<Character>>,
    assets: Res<MineAssets>,
) {
    for (timer, damage_range, parent) in &mut mine_droppers {
        if timer.just_finished() {
            if let Ok((Transform { translation, .. }, target)) = characters.get(**parent) {
                let image = assets.mine_texture.clone();
                let atlas = assets.mine_atlas_layout.clone().into();
                commands.spawn((
                    Mine,
                    DamagerParams {
                        damage_range: *damage_range,
                        transform: Transform::from_xyz(translation.x, translation.y, 12.),
                        collision_groups: Damager::collision_groups(*target),
                    },
                    Sprite::from_atlas_image(image, atlas),
                ));
            }
        }
    }
}

fn mine_explosion(
    mut commands: Commands,
    mut mines: Query<(
        Entity,
        &mut MineExplodeTimer,
        &DamageRange,
        &Transform,
        &CollisionGroups,
    )>,
    time: Res<Time>,
    assets: Res<MineAssets>,
) {
    for (entity, mut timer, &damage_range, &transform, &collision_groups) in &mut mines {
        timer.tick(time.delta());
        if timer.just_finished() {
            commands.entity(entity).despawn_recursive();

            // Spawn explosion
            let image = assets.explosion_texture.clone();
            let atlas = assets.explosion_atlas_layout.clone().into();
            commands.spawn((
                MineExplosion,
                DamagerParams {
                    damage_range,
                    collision_groups,
                    transform,
                },
                Sprite::from_atlas_image(image, atlas),
            ));
        }
    }
}

fn despawn_explosion(
    mut commands: Commands,
    explosions: Query<(Entity, &OneShotAnimation), With<MineExplosion>>,
) {
    for (entity, animation) in &explosions {
        if animation.finished() {
            commands.entity(entity).despawn();
        }
    }
}
