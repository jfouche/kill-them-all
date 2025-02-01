use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use super::{
    animation::AnimationTimer,
    character::{BaseLife, BaseMovementSpeed, Character, Target},
    damage::HitDamageRange,
    GROUP_ALL, GROUP_ENEMY, GROUP_ITEM,
};

///
///  Assets of a single monster
///
pub struct MonsterAssets {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

impl MonsterAssets {
    pub fn sprite(&self) -> Sprite {
        Sprite {
            image: self.texture.clone(),
            texture_atlas: Some(self.atlas_layout.clone().into()),
            ..Default::default()
        }
    }
}

///
///  Assets of all monsters
///
#[derive(Resource, Deref, DerefMut)]
pub struct AllMonsterAssets(pub Vec<MonsterAssets>);

impl FromWorld for AllMonsterAssets {
    fn from_world(world: &mut World) -> Self {
        let atlas_layout = world.add_asset(TextureAtlasLayout::from_grid(
            UVec2::new(16, 16),
            4,
            4,
            None,
            None,
        ));

        AllMonsterAssets(vec![
            // Monster kind 1
            MonsterAssets {
                texture: world.load_asset("characters/Cyclope/SpriteSheet.png"),
                atlas_layout: atlas_layout.clone(),
            },
            // Monster kind 2
            MonsterAssets {
                texture: world.load_asset("characters/Skull/SpriteSheet.png"),
                atlas_layout: atlas_layout.clone(),
            },
            // Monster kind 3
            MonsterAssets {
                texture: world.load_asset("characters/DragonYellow/SpriteSheet.png"),
                atlas_layout: atlas_layout.clone(),
            },
        ])
    }
}

impl AllMonsterAssets {
    pub fn sprite(&self, kind: usize) -> Sprite {
        self.get(kind)
            .expect("Monster type out of range !")
            .sprite()
    }
}

/// Monster view range
#[derive(Component, Clone, Copy, Deref, Reflect)]
pub struct ViewRange(pub f32);

impl Default for ViewRange {
    fn default() -> Self {
        ViewRange(250.)
    }
}

/// Monster level
#[derive(Component, Default, Clone, Copy, Deref, Reflect)]
pub struct MonsterLevel(pub u16);

///
/// Generic Monster component
///
#[derive(Component, Default)]
#[require(
    Character,
    Target(|| Target::Player),
    ViewRange,
    MonsterRarity,
    XpOnDeath,
    HitDamageRange,
    Sprite,
    AnimationTimer,
    Collider(|| Collider::cuboid(8., 8.)),
    CollisionGroups(|| CollisionGroups::new(GROUP_ENEMY, GROUP_ALL & !GROUP_ITEM))
)]
pub struct Monster;

#[derive(Component)]
#[require(
    Name(|| Name::new("Monster#1")),
    Monster,
    BaseLife(|| BaseLife(2.)),
    BaseMovementSpeed(||BaseMovementSpeed(50.)),
)]
pub struct MonsterType1;

#[derive(Component)]
#[require(
    Name(|| Name::new("Monster#2")),
    Monster,
    BaseLife(|| BaseLife(3.)),
    BaseMovementSpeed(||BaseMovementSpeed(40.)),
)]
pub struct MonsterType2;

#[derive(Component)]
#[require(
    Name(|| Name::new("Monster#3")),
    Monster,
    BaseLife(|| BaseLife(4.)),
    BaseMovementSpeed(||BaseMovementSpeed(30.)),
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
    pub rarity: MonsterRarity,
    pub kind: usize,
    pub level: u16,
}

impl MonsterSpawnParams {
    pub fn generate(level: u16, rng: &mut ThreadRng) -> Self {
        // Rarity
        let rarity = match rng.random_range(0..35) {
            0 => MonsterRarity::Rare,
            _ => MonsterRarity::Normal,
        };

        // Kind
        let kind = rng.random_range(0..MONSTER_KIND_COUNT);

        // Create the params
        MonsterSpawnParams {
            kind,
            rarity,
            level,
        }
    }

    pub fn scale(&self) -> Vec3 {
        let scale = match self.rarity {
            MonsterRarity::Normal => 1.,
            MonsterRarity::Rare => 2.,
        };
        Vec3::new(scale, scale, 1.)
    }

    pub fn xp_on_death(&self) -> XpOnDeath {
        let xp = match self.rarity {
            MonsterRarity::Normal => 1,
            MonsterRarity::Rare => 4,
        };
        let multiplier = u32::from(self.level) + 1;
        XpOnDeath(xp * multiplier)
    }

    pub fn hit_damage_range(&self) -> HitDamageRange {
        let (min, max) = match self.rarity {
            MonsterRarity::Normal => (1., 2.),
            MonsterRarity::Rare => (2., 4.),
        };
        let multiplier = (self.level + 1) as f32;
        let min = min * multiplier;
        let max = max * multiplier;
        HitDamageRange::new(min, max)
    }
}

impl From<&MonsterSpawnParams> for XpOnDeath {
    fn from(value: &MonsterSpawnParams) -> Self {
        value.xp_on_death()
    }
}

impl From<&MonsterSpawnParams> for HitDamageRange {
    fn from(value: &MonsterSpawnParams) -> Self {
        value.hit_damage_range()
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
        MonsterSpawnTimer(Timer::from_seconds(3., TimerMode::Once))
    }
}

///
/// Event to notify a monster died
///
#[derive(Event)]
pub struct MonsterDeathEvent {
    pub pos: Vec3,
    pub xp: u32,
    pub mlevel: u16,
}
