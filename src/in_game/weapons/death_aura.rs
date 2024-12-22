use super::*;
use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};
use bevy_rapier2d::prelude::*;

///
/// Death aura weapon
///
#[derive(Component)]
#[require(
    Name(|| Name::new("DeathAura")),
    Weapon,
    Damager,
    Transform,
    Visibility,
    Collider(|| Collider::ball(16.))
)]
pub struct DeathAura;

///
/// Assets for [DeathAura]
///
#[derive(Resource)]
pub struct DeathAuraAssets {
    mesh: Handle<Mesh>,
    material: Handle<DeathAuraMaterial>,
}

impl FromWorld for DeathAuraAssets {
    fn from_world(world: &mut World) -> Self {
        DeathAuraAssets {
            mesh: world.add_asset(Circle::new(16.)),
            material: world.add_asset(DeathAuraMaterial {
                color: LinearRgba::BLUE,
            }),
        }
    }
}

///
///  Material for [DeathAura]
///
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct DeathAuraMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for DeathAuraMaterial {
    fn fragment_shader() -> ShaderRef {
        "death_aura.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

///
///  Plugin for the [DeathAura] weapon
///
pub struct DeathAuraPlugin;

impl Plugin for DeathAuraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<DeathAuraMaterial>::default())
            .init_resource::<DeathAuraAssets>()
            .add_observer(init_death_aura);
    }
}

fn init_death_aura(
    trigger: Trigger<OnAdd, DeathAura>,
    mut commands: Commands,
    assets: Res<DeathAuraAssets>,
) {
    commands.entity(trigger.entity()).insert((
        Mesh2d(assets.mesh.clone()),
        MeshMaterial2d(assets.material.clone()),
    ));
}
