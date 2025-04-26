use crate::components::{
    character::Target,
    damage::Damager,
    skills::death_aura::{DeathAura, DeathAuraAssets, DeathAuraMaterial},
};
use bevy::{prelude::*, sprite::Material2dPlugin};

///
///  Plugin for the [DeathAura] weapon
///
pub struct DeathAuraPlugin;

impl Plugin for DeathAuraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<DeathAuraMaterial>::default())
            .init_resource::<DeathAuraAssets>()
            .add_observer(init_render)
            .add_observer(init_target);
    }
}

fn init_render(
    trigger: Trigger<OnAdd, DeathAura>,
    mut commands: Commands,
    assets: Res<DeathAuraAssets>,
) {
    commands.entity(trigger.target()).insert((
        Mesh2d(assets.mesh.clone()),
        MeshMaterial2d(assets.material.clone()),
    ));
}

fn init_target(
    trigger: Trigger<OnAdd, ChildOf>,
    mut commands: Commands,
    death_auras: Query<&ChildOf, With<DeathAura>>,
    targets: Query<&Target>,
) {
    if let Ok(child_of) = death_auras.get(trigger.target()) {
        if let Ok(&target) = targets.get(child_of.parent()) {
            commands
                .entity(trigger.target())
                .insert(Damager::collision_groups(target));
        }
    }
}
