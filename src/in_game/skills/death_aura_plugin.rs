use crate::components::{
    character::{Character, Target},
    damage::Damager,
    item::update_item_info,
    skills::death_aura::{DeathAura, DeathAuraAssets, DeathAuraBook, DeathAuraMaterial},
    world_map::LAYER_DAMAGER,
};
use bevy::{prelude::*, sprite::Material2dPlugin};
use bevy_rapier2d::prelude::CollisionGroups;

///
///  Plugin for the [DeathAura] weapon
///
pub struct DeathAuraPlugin;

impl Plugin for DeathAuraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<DeathAuraMaterial>::default())
            .init_resource::<DeathAuraAssets>()
            .add_observer(update_item_info::<DeathAuraBook>())
            .add_observer(on_equip)
            .add_observer(on_unequip);
    }
}

fn on_equip(
    trigger: Trigger<OnAdd, DeathAura>,
    mut death_auras: Query<
        (
            &mut Transform,
            &mut Mesh2d,
            &mut MeshMaterial2d<DeathAuraMaterial>,
            &mut CollisionGroups,
            &ChildOf,
        ),
        With<DeathAura>,
    >,
    targets: Query<&Target, With<Character>>,
    assets: Res<DeathAuraAssets>,
) {
    if let Ok((mut transform, mut mesh, mut material, mut collision_groups, &ChildOf(parent))) =
        death_auras.get_mut(trigger.target())
    {
        if let Ok(&target) = targets.get(parent) {
            info!("Equip DeathAura");
            transform.translation = vec3(0., 0., LAYER_DAMAGER);
            mesh.0 = assets.mesh.clone();
            material.0 = assets.material.clone();
            *collision_groups = Damager::collision_groups(target);
        }
    }
}

fn on_unequip(
    trigger: Trigger<OnRemove, ChildOf>,
    mut death_auras: Query<
        (
            &mut Mesh2d,
            &mut MeshMaterial2d<DeathAuraMaterial>,
            &mut CollisionGroups,
        ),
        With<DeathAura>,
    >,
) {
    if let Ok((mut mesh, mut material, mut collision_groups)) =
        death_auras.get_mut(trigger.target())
    {
        *mesh = Mesh2d::default();
        *material = MeshMaterial2d::default();
        *collision_groups = CollisionGroups::default();
    }
}
