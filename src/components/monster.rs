use super::*;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

///
///  Assets of a single monster
///
pub struct MonsterAssets {
    pub texture: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

///
///  Assets of all monsters
///
#[derive(Resource, Default, Deref, DerefMut)]
pub struct AllMonsterAssets(pub Vec<MonsterAssets>);

///
/// Assets used to show where the monster will spawn
///
#[derive(Resource)]
pub struct SpawningMonsterAssets {
    pub mesh: Handle<Mesh>,
    pub color: Handle<ColorMaterial>,
}

impl From<&SpawningMonsterAssets> for Mesh2d {
    fn from(value: &SpawningMonsterAssets) -> Self {
        Mesh2d(value.mesh.clone())
    }
}

impl From<&SpawningMonsterAssets> for MeshMaterial2d<ColorMaterial> {
    fn from(value: &SpawningMonsterAssets) -> Self {
        MeshMaterial2d(value.color.clone())
    }
}

///
/// Generic Monster component
///
#[derive(Component, Default)]
#[require(
    Character,
    MonsterRarity,
    XpOnDeath,
    DamageRange,
    Sprite,
    AnimationTimer
)]
pub struct Monster;

#[derive(Component)]
#[require(
    Name(|| Name::new("Monster#1")),
    Monster,
    BaseLife(|| BaseLife(2.)),
    BaseMovementSpeed(||BaseMovementSpeed(90.)),
)]
pub struct MonsterType1;

#[derive(Component)]
#[require(
    Name(|| Name::new("Monster#2")),
    Monster,
    BaseLife(|| BaseLife(3.)),
    BaseMovementSpeed(||BaseMovementSpeed(80.)),
)]
pub struct MonsterType2;

#[derive(Component)]
#[require(
    Name(|| Name::new("Monster#3")),
    Monster,
    BaseLife(|| BaseLife(4.)),
    BaseMovementSpeed(||BaseMovementSpeed(70.)),
)]
pub struct MonsterType3;

// TODO: use enum
const MONSTER_KIND_COUNT: usize = 3;

#[derive(Component, Default, Clone, Copy, Reflect)]
pub enum MonsterRarity {
    #[default]
    Normal,
    Rare,
}

///
/// Contains the monster informations to spawn
///
#[derive(Component, Default, Reflect)]
pub struct MonsterSpawnParams {
    pub pos: Vec2,
    pub rarity: MonsterRarity,
    pub kind: usize,
    pub level: u16,
}

impl MonsterSpawnParams {
    pub fn generate(level: u16, rng: &mut ThreadRng) -> Self {
        // Position
        let x: f32 = rng.gen_range(-150. ..150.);
        let y: f32 = rng.gen_range(-100. ..100.);

        // Rarity
        let rarity = match rng.gen_range(0..5) {
            0 => MonsterRarity::Rare,
            _ => MonsterRarity::Normal,
        };

        // Kind
        let kind = rand::thread_rng().gen_range(0..MONSTER_KIND_COUNT);

        // Create the params
        MonsterSpawnParams {
            pos: Vec2::new(x, y),
            kind,
            rarity,
            level,
        }
    }

    fn size(&self) -> Vec2 {
        match self.rarity {
            MonsterRarity::Normal => Vec2::new(16.0, 16.0),
            MonsterRarity::Rare => Vec2::new(32.0, 32.0),
        }
    }
}

impl From<&MonsterSpawnParams> for Transform {
    fn from(value: &MonsterSpawnParams) -> Self {
        Transform::from_xyz(value.pos.x, value.pos.y, 10.)
    }
}

///
/// Utility to simplify components initialization
///
pub struct MonsterSpawnParamsAndAssets<'a> {
    pub params: &'a MonsterSpawnParams,
    pub assets: &'a AllMonsterAssets,
}

impl From<&MonsterSpawnParamsAndAssets<'_>> for MonsterRarity {
    fn from(value: &MonsterSpawnParamsAndAssets) -> Self {
        value.params.rarity
    }
}

impl From<&MonsterSpawnParamsAndAssets<'_>> for Sprite {
    fn from(value: &MonsterSpawnParamsAndAssets) -> Self {
        let assets = value
            .assets
            .get(value.params.kind)
            .expect("Monster type out of range !");

        Sprite {
            image: assets.texture.clone(),
            texture_atlas: Some(assets.texture_atlas_layout.clone().into()),
            custom_size: Some(value.params.size()),
            ..Default::default()
        }
    }
}

impl From<&MonsterSpawnParamsAndAssets<'_>> for XpOnDeath {
    fn from(value: &MonsterSpawnParamsAndAssets) -> Self {
        match value.params.rarity {
            MonsterRarity::Normal => XpOnDeath(1),
            MonsterRarity::Rare => XpOnDeath(4),
        }
    }
}

impl From<&MonsterSpawnParamsAndAssets<'_>> for DamageRange {
    fn from(value: &MonsterSpawnParamsAndAssets) -> Self {
        let (min, max) = match value.params.rarity {
            MonsterRarity::Normal => (1., 2.),
            MonsterRarity::Rare => (2., 4.),
        };
        let multiplier = (value.params.level + 1) as f32;
        let min = min * multiplier;
        let max = max * multiplier;
        DamageRange::new(min, max)
    }
}

impl From<&MonsterSpawnParamsAndAssets<'_>> for Collider {
    fn from(value: &MonsterSpawnParamsAndAssets<'_>) -> Self {
        Collider::cuboid(value.params.size().x / 2., value.params.size().y / 2.)
    }
}

///
/// Experience given to player when the monster is killed
///
#[derive(Component, Default, Deref)]
pub struct XpOnDeath(pub u32);

///
/// Component to inform that a monster will spawn
///
#[derive(Component)]
#[require(
    Name(|| Name::new("MonsterFuturePos")),
    MonsterSpawnTimer,
    Transform,
    Mesh2d,
    MeshMaterial2d<ColorMaterial>,
    MonsterSpawnParams
)]
pub struct MonsterFuturePos;

///
/// Timer between spawning information and real monster spawn
///
#[derive(Component, Deref, DerefMut, Reflect)]
pub struct MonsterSpawnTimer(pub Timer);

impl Default for MonsterSpawnTimer {
    fn default() -> Self {
        MonsterSpawnTimer(Timer::from_seconds(1., TimerMode::Once))
    }
}

///
/// Event to notify a monster died
///
#[derive(Event)]
pub struct MonsterDeathEvent {
    pub pos: Vec3,
    pub xp: u32,
}
