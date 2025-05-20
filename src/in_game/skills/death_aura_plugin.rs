use crate::{
    components::{
        character::{Character, Target},
        damage::Damager,
        skills::death_aura::{DeathAura, DeathAuraAssets, DeathAuraMaterial},
        world_map::LAYER_DAMAGER,
    },
    schedule::InGameState,
};
use avian2d::prelude::{CollisionLayers, Position};
use bevy::{prelude::*, sprite::Material2dPlugin};

///
///  Plugin for the [DeathAura] weapon
///
pub struct DeathAuraPlugin;

impl Plugin for DeathAuraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<DeathAuraMaterial>::default())
            .init_resource::<DeathAuraAssets>()
            .add_systems(
                Update,
                fix_position
                    .after(TransformSystem::TransformPropagate)
                    .run_if(in_state(InGameState::Running)),
            )
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
            &mut CollisionLayers,
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
            &mut CollisionLayers,
        ),
        With<DeathAura>,
    >,
) {
    if let Ok((mut mesh, mut material, mut collision_groups)) =
        death_auras.get_mut(trigger.target())
    {
        *mesh = Mesh2d::default();
        *material = MeshMaterial2d::default();
        *collision_groups = CollisionLayers::default();
    }
}

fn fix_position(
    mut death_auras: Query<(&mut Position, &ChildOf), With<DeathAura>>,
    characters: Query<&Position, (With<Character>, Without<DeathAura>)>,
) {
    for (mut aura_pos, &ChildOf(parent)) in &mut death_auras {
        if let Ok(&pos) = characters.get(parent) {
            *aura_pos = pos;
        }
    }
}
