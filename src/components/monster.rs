use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use super::{
    affix::IncreaseAttackSpeed,
    animation::AnimationTimer,
    character::{BaseLife, BaseMovementSpeed, Character, Target},
    damage::HitDamageRange,
    world_map::LAYER_MONSTER,
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
    Target::Player,
    ViewRange,
    MonsterRarity,
    XpOnDeath,
    HitDamageRange,
    Sprite,
    AnimationTimer,
    Collider::cuboid(8., 8.),
    CollisionGroups::new(GROUP_ENEMY, GROUP_ALL & !GROUP_ITEM)
)]
pub struct Monster;

#[derive(Component)]
#[require(Name::new("Monster#1"), Monster, BaseLife(2.), BaseMovementSpeed(50.))]
pub struct MonsterType1;

impl MonsterType1 {
    pub fn bundle(builder: MonsterBuilder, pos: Vec2, assets: &AllMonsterAssets) -> impl Bundle {
        (MonsterType1, builder.bundle(pos, assets))
    }
}

#[derive(Component)]
#[require(Name::new("Monster#2"), Monster, BaseLife(3.), BaseMovementSpeed(40.))]
pub struct MonsterType2;

impl MonsterType2 {
    pub fn bundle(builder: MonsterBuilder, pos: Vec2, assets: &AllMonsterAssets) -> impl Bundle {
        (MonsterType2, builder.bundle(pos, assets))
    }
}

#[derive(Component)]
#[require(Name::new("Monster#3"), Monster, BaseLife(4.), BaseMovementSpeed(30.))]
pub struct MonsterType3;

impl MonsterType3 {
    pub fn bundle(builder: MonsterBuilder, pos: Vec2, assets: &AllMonsterAssets) -> impl Bundle {
        (MonsterType3, builder.bundle(pos, assets))
    }
}

// TODO: use enum
const MONSTER_KIND_COUNT: usize = 3;

#[derive(Component, Default, Clone, Copy, Reflect)]
pub enum MonsterRarity {
    #[default]
    Normal,
    Rare,
}

/// Event sent (use [EventReader] to read) when the monsters can be spawn
#[derive(Event, Default)]
pub struct SpawnMonstersEvent {
    pub mlevel: u16,
    pub monsters: Vec<(Vec2, u16)>,
}

///
/// Contains the monster informations to spawn
///
#[derive(Component, Default, Reflect)]
pub struct MonsterBuilder {
    pub rarity: MonsterRarity,
    pub kind: usize,
    pub level: u16,
}

impl MonsterBuilder {
    pub fn generate(level: u16, rng: &mut ThreadRng) -> Self {
        // Rarity
        let percent = match level {
            0..1 => 0,
            1..4 => 10,
            4..8 => 20,
            8..15 => 30,
            15..25 => 40,
            25..45 => 50,
            45..65 => 60,
            _ => 70,
        };
        let rarity = if rng.random_range(0..100) >= percent {
            MonsterRarity::Normal
        } else {
            MonsterRarity::Rare
        };

        // Kind
        let kind = rng.random_range(0..MONSTER_KIND_COUNT);

        // Create the params
        MonsterBuilder {
            kind,
            rarity,
            level,
        }
    }

    pub fn bundle(&self, pos: Vec2, assets: &AllMonsterAssets) -> impl Bundle {
        let translation = pos.extend(LAYER_MONSTER);
        let scale = self.scale();
        let xp = self.xp_on_death();
        let damage_range = self.hit_damage_range();
        (
            MonsterLevel(self.level),
            self.rarity,
            assets.sprite(self.kind),
            Transform::from_translation(translation).with_scale(scale),
            XpOnDeath(xp),
            HitDamageRange::new(damage_range.0, damage_range.1),
            children![IncreaseAttackSpeed(-60.)],
        )
    }

    fn scale(&self) -> Vec3 {
        let scale = match self.rarity {
            MonsterRarity::Normal => 1.,
            MonsterRarity::Rare => 2.,
        };
        Vec3::new(scale, scale, 1.)
    }

    fn xp_on_death(&self) -> u32 {
        let xp = match self.rarity {
            MonsterRarity::Normal => 1,
            MonsterRarity::Rare => 4,
        };
        let multiplier = u32::from(self.level) + 1;
        xp * multiplier
    }

    pub fn hit_damage_range(&self) -> (f32, f32) {
        let (min, max) = match self.rarity {
            MonsterRarity::Normal => (1., 2.),
            MonsterRarity::Rare => (2., 4.),
        };
        let multiplier = (self.level + 1) as f32;
        let min = min * multiplier;
        let max = max * multiplier;
        (min, max)
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
    Name::new("MonsterFuturePos"),
    MonsterSpawnTimer,
    Transform,
    Mesh2d,
    MeshMaterial2d<ColorMaterial>,
    MonsterBuilder
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
