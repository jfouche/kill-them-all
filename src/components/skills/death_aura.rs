use crate::components::{
    damage::{BaseDamageOverTime, Damager},
    item::{ItemDescriptor, ItemRarity},
    skills::{AffectedByAreaOfEffect, Skill, SkillBook},
};
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};
use bevy_rapier2d::prelude::*;

use super::SkillOfBook;

///
/// Death aura weapon
///
#[derive(Component, Default)]
#[require(Name::new("DeathAuraBook"), SkillBook)]
pub struct DeathAuraBook;

impl ItemDescriptor for DeathAuraBook {
    fn title(&self) -> String {
        "Death aura".into()
    }

    fn description(&self) -> String {
        r#"Aura that damages over time
Affected by AOE affixes"#
            .into()
    }

    fn tile_index(&self, _rarity: ItemRarity) -> usize {
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
    Collider::ball(32.),
    CollisionGroups
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
