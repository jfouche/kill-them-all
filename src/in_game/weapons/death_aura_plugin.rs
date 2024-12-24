use super::*;
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
    commands.entity(trigger.entity()).insert((
        Mesh2d(assets.mesh.clone()),
        MeshMaterial2d(assets.material.clone()),
    ));
}

fn init_target(
    trigger: Trigger<OnAdd, Parent>,
    mut commands: Commands,
    death_auras: Query<&Parent, With<DeathAura>>,
    targets: Query<&Target>,
) {
    if let Ok(parent) = death_auras.get(trigger.entity()) {
        if let Ok(&target) = targets.get(**parent) {
            commands
                .entity(trigger.entity())
                .insert(Damager::collision_groups(target));
        }
    }
}
