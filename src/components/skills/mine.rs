use super::{Skill, SkillBook, SkillOfBook};
use crate::components::{
    animation::{CyclicAnimation, OneShotAnimation},
    damage::{Damager, HitDamageRange},
    equipment::weapon::BaseAttackSpeed,
    item::{ItemDescriptor, ItemRarity},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

///
/// |SkillBook] for [MineDropper]
///
#[derive(Component, Default)]
#[require(Name::new("MineDropperBook"), SkillBook)]
pub struct MineDropperBook;

impl ItemDescriptor for MineDropperBook {
    fn title(&self) -> String {
        "Mine dropper".into()
    }

    fn description(&self) -> String {
        "Drop mine which explodes".into()
    }

    fn tile_index(&self, _rarity: ItemRarity) -> usize {
        21
    }
}

impl SkillOfBook for MineDropperBook {
    type Skill = MineDropper;
}

///
/// |Skill] that drop a mine regularly
///
#[derive(Component, Default)]
#[require(
    Name::new("MineDropper"),
    Skill,
    HitDamageRange::new(1., 5.),
    BaseAttackSpeed(0.6)
)]
pub struct MineDropper;

///
/// Mine
///
#[derive(Component)]
#[require(
    Name::new("Mine"),
    Damager,
    Collider::ball(8.),
    MineExplodeTimer,
    Sprite,
    CyclicAnimation::new(0..2)
)]
pub struct Mine;

#[derive(Component, Deref, DerefMut, Reflect)]
pub struct MineExplodeTimer(Timer);

impl Default for MineExplodeTimer {
    fn default() -> Self {
        MineExplodeTimer(Timer::from_seconds(1.5, TimerMode::Once))
    }
}

#[derive(Component)]
#[require(
    Damager,
    Collider::ball(16.),
    Sprite,
    OneShotAnimation::new(0..8)
)]
pub struct MineExplosion;

#[derive(Resource)]
pub struct MineAssets {
    pub mine_texture: Handle<Image>,
    pub mine_atlas_layout: Handle<TextureAtlasLayout>,
    pub explosion_texture: Handle<Image>,
    pub explosion_atlas_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for MineAssets {
    fn from_world(world: &mut World) -> Self {
        let mine_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2, 1, None, None);
        let explosion_atlas_layout =
            TextureAtlasLayout::from_grid(UVec2::new(32, 32), 8, 1, None, None);

        MineAssets {
            mine_texture: world.load_asset("mine.png"),
            mine_atlas_layout: world.add_asset(mine_atlas_layout),
            explosion_texture: world.load_asset("mine_explosion.png"),
            explosion_atlas_layout: world.add_asset(explosion_atlas_layout),
        }
    }
}
