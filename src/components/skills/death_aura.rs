use crate::components::{
    damage::{BaseDamageOverTime, Damager},
    skills::{AffectedByAreaOfEffect, Skill, SkillBook, SkillBookUI},
};
use avian2d::prelude::*;
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};

use super::SkillOfBook;

///
/// Death aura weapon
///
#[derive(Component, Default)]
#[require(Name::new("DeathAuraBook"), SkillBook)]
pub struct DeathAuraBook;

impl SkillBookUI for DeathAuraBook {
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

impl SkillOfBook for DeathAuraBook {
    type Skill = DeathAura;
}

///
/// Death aura weapon
///
#[derive(Component, Default)]
#[require(
    Name::new("DeathAura"),
    Skill,
    AffectedByAreaOfEffect,
    Damager,
    BaseDamageOverTime(3.),
    Transform,
    Visibility,
    Mesh2d,
    MeshMaterial2d<DeathAuraMaterial>,
    Collider::circle(32.),
    CollisionLayers
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
