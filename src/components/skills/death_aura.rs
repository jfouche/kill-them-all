use crate::components::*;
use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};
use bevy_rapier2d::prelude::*;
use skills::Skill;

///
/// Death aura weapon
///
#[derive(Component)]
#[require(
    Name(|| Name::new("DeathAura")),
    Skill,
    Damager,
    BaseDamageOverTime(|| BaseDamageOverTime(1.)),
    Transform,
    Visibility,
    Collider(|| Collider::ball(32.))
)]
pub struct DeathAura;

///
/// Assets for [DeathAura]
///
#[derive(Resource)]
pub struct DeathAuraAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<DeathAuraMaterial>,
}

impl FromWorld for DeathAuraAssets {
    fn from_world(world: &mut World) -> Self {
        DeathAuraAssets {
            mesh: world.add_asset(Circle::new(32.)),
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
pub struct DeathAuraMaterial {
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
