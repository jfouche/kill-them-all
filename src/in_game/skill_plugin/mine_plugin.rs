use crate::{
    components::{
        animation::OneShotAnimation,
        character::{Character, Target},
        damage::{Damager, DamagerParams, HitDamageRange},
        despawn_all,
        equipment::weapon::AttackTimer,
        skills::mine::{Mine, MineAssets, MineDropper, MineExplodeTimer, MineExplosion},
    },
    in_game::{GameRunningSet, GameState},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionGroups;

pub struct MinePlugin;

impl Plugin for MinePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<MineAssets>()
            .add_systems(OnExit(GameState::InGame), despawn_all::<Mine>)
            .add_systems(OnExit(GameState::InGame), despawn_all::<MineExplosion>)
            .add_systems(
                Update,
                (drop_mine, mine_explosion, despawn_explosion).in_set(GameRunningSet::EntityUpdate),
            );
    }
}

fn drop_mine(
    mut commands: Commands,
    mut mine_droppers: Query<(&AttackTimer, &HitDamageRange, &Parent), With<MineDropper>>,
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
                    *damage_range,
                    DamagerParams {
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
        &HitDamageRange,
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
                damage_range,
                DamagerParams {
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
