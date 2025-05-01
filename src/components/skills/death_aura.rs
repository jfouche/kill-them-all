use crate::components::*;
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};
use bevy_rapier2d::prelude::*;
use damage::{BaseDamageOverTime, Damager};
use skills::{AffectedByAreaOfEffect, SkillBook, SkillUI};

///
/// Death aura weapon
///
#[derive(Component, Default)]
#[require(
    Name::new("DeathAura"),
    SkillBook,
    AffectedByAreaOfEffect,
    Damager,
    BaseDamageOverTime(3.),
    Transform,
    Visibility,
    Mesh2d,
    MeshMaterial2d<DeathAuraMaterial>,
    Collider::ball(32.),
    CollisionGroups
)]
pub struct DeathAura;

impl SkillUI for DeathAura {
    fn title() -> String {
        "Death aura".into()
    }

    fn label() -> String {
        r#"Aura that damages over time
Affected by AOE affixes"#
            .into()
    }

    fn tile_index() -> usize {
        61
    }
}

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
