use super::*;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

pub struct MonsterAssets {
    pub texture: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct AllMonsterAssets(pub Vec<MonsterAssets>);

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

#[derive(Component, Default, Clone, Copy)]
pub enum MonsterRarity {
    #[default]
    Normal,
    Rare,
}

/// Contains the monster informations to spawn
#[derive(Default)]
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

    // pub fn damage_range(&self) -> DamageRange {
    //     let (min, max) = match self.rarity {
    //         MonsterRarity::Normal => (1., 2.),
    //         MonsterRarity::Rare => (2., 4.),
    //     };
    //     let multiplier = (self.level + 1) as f32;
    //     let min = min * multiplier;
    //     let max = max * multiplier;
    //     DamageRange::new(min, max)
    // }

    // pub fn xp(&self) -> XpOnDeath {
    //     let xp = match self.rarity {
    //         MonsterRarity::Normal => 1u32,
    //         MonsterRarity::Rare => 3,
    //     };
    //     XpOnDeath(xp * (self.level + 1) as u32)
    // }

    // pub fn movement_speed(&self) -> BaseMovementSpeed {
    //     match self.rarity {
    //         MonsterRarity::Normal => BaseMovementSpeed(90.),
    //         MonsterRarity::Rare => BaseMovementSpeed(70.),
    //     }
    // }

    // pub fn life(&self) -> BaseLife {
    //     let life = match self.rarity {
    //         MonsterRarity::Normal => 2.,
    //         MonsterRarity::Rare => 5.,
    //     };
    //     // 5% increase life per level
    //     let incr = self.level as f32 * 5.;
    //     BaseLife(life * (100. + incr) / 100.)
    // }
}

pub struct MonsterSpawningParamsAndAssets<'a> {
    pub params: &'a MonsterSpawnParams,
}

impl From<&MonsterSpawningParamsAndAssets<'_>> for Sprite {
    fn from(value: &MonsterSpawningParamsAndAssets) -> Self {
        Sprite {
            color: Color::srgba(0.8, 0.3, 0.3, 0.2),
            custom_size: Some(value.params.size()),
            ..Default::default()
        }
    }
}

pub struct MonsterSpawnParamsAndAssets<'a> {
    pub params: &'a MonsterSpawnParams,
    pub assets: &'a AllMonsterAssets,
}

impl From<&MonsterSpawnParamsAndAssets<'_>> for MonsterRarity {
    fn from(value: &MonsterSpawnParamsAndAssets) -> Self {
        value.params.rarity
    }
}
impl From<&MonsterSpawnParamsAndAssets<'_>> for Transform {
    fn from(value: &MonsterSpawnParamsAndAssets) -> Self {
        Transform::from_xyz(value.params.pos.x, value.params.pos.y, 10.)
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

#[derive(Component, Default, Deref)]
pub struct XpOnDeath(pub u32);

#[derive(Resource)]
pub struct MonsterSpawningConfig {
    pub timer: Timer,
    pub enemy_count: u16,
}

impl Default for MonsterSpawningConfig {
    fn default() -> Self {
        MonsterSpawningConfig {
            timer: Timer::from_seconds(6., TimerMode::Repeating),
            enemy_count: 3,
        }
    }
}

#[derive(Component)]
pub struct MonsterSpawnConfig {
    pub timer: Timer,
    pub params: MonsterSpawnParams,
}

impl Default for MonsterSpawnConfig {
    fn default() -> Self {
        MonsterSpawnConfig {
            timer: Timer::from_seconds(1., TimerMode::Once),
            params: MonsterSpawnParams::default(),
        }
    }
}

impl MonsterSpawnConfig {
    pub fn new(params: MonsterSpawnParams) -> Self {
        MonsterSpawnConfig {
            timer: Timer::from_seconds(1., TimerMode::Once),
            params,
        }
    }
}

#[derive(Component)]
#[require(
    Name(|| Name::new("MonsterFuturePos")),
    Sprite,
    MonsterSpawnConfig,
    MonsterSpawningTimer
)]
pub struct MonsterFuturePos;

#[derive(Component)]
pub struct MonsterSpawningTimer(pub Timer);

impl Default for MonsterSpawningTimer {
    fn default() -> Self {
        MonsterSpawningTimer(Timer::from_seconds(1., TimerMode::Once))
    }
}

// #[derive(Bundle)]
// pub struct MonsterFuturePosBundle {
//     tag: MonsterFuturePos,
//     name: Name,
//     sprite: Sprite,
//     transform: Transform,
//     config: MonsterSpawnConfig,
// }

impl MonsterFuturePosBundle {
    pub fn new(params: MonsterSpawnParams) -> Self {
        MonsterFuturePosBundle {
            tag: MonsterFuturePos,
            name: Name::new("MonsterFuturePos"),
            sprite: Sprite {
                color: Color::srgba(0.8, 0.3, 0.3, 0.2),
                custom_size: Some(params.size()),
                ..Default::default()
            },
            transform: Transform::from_xyz(params.pos.x, params.pos.y, 1.),
            config: MonsterSpawnConfig::new(params),
        }
    }
}

/// Event to notify a monster died
#[derive(Event)]
pub struct MonsterDeathEvent {
    pub pos: Vec3,
    pub xp: u32,
}
