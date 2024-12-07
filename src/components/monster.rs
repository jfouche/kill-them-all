use super::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{rngs::ThreadRng, Rng};

pub struct MonsterAssets {
    pub texture: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct AllMonsterAssets(pub Vec<MonsterAssets>);

#[derive(Component)]
pub struct Monster;

#[derive(Bundle)]
pub struct MonsterBundle {
    tag: Monster,
    name: Name,
    // bevy view
    sprite: Sprite,
    transform: Transform,
    animation_timer: AnimationTimer,
    // skills
    skills: SkillsBundle,
    xp_on_death: XpOnDeath,
    damage_range: DamageRange,
    // physics
    body: RigidBody,
    velocity: Velocity,
    collider: Collider,
    collision_groups: CollisionGroups,
    locked_axes: LockedAxes,
}

impl Default for MonsterBundle {
    fn default() -> Self {
        MonsterBundle {
            tag: Monster,
            name: Name::new("Monster"),
            sprite: Sprite::default(),
            transform: Transform::default(),
            animation_timer: AnimationTimer::default(),
            skills: SkillsBundle::default(),
            xp_on_death: XpOnDeath(1),
            damage_range: DamageRange(1. ..=2.),
            body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            collider: Collider::default(),
            collision_groups: CollisionGroups::new(GROUP_ENEMY, Group::ALL & !GROUP_BONUS),
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }
}

impl MonsterBundle {
    pub fn new(assets: &MonsterAssets, params: &MonsterSpawnParams) -> Self {
        let size = params.size();
        MonsterBundle {
            skills: SkillsBundle {
                movement_speed: params.movement_speed(),
                life: params.life(),
                ..Default::default()
            },
            xp_on_death: params.xp(),
            damage_range: params.damage_range(),
            sprite: Sprite {
                image: assets.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.texture_atlas_layout.clone(),
                    index: 0,
                }),
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_xyz(params.pos.x, params.pos.y, 10.),
            collider: Collider::cuboid(size.x / 2., size.y / 2.),
            ..Default::default()
        }
    }
}

// TODO: use enum
const MONSTER_KIND_COUNT: usize = 3;

pub enum MonsterRarity {
    Normal,
    Rare,
}

/// Contains the monster informations to spawn
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

    pub fn damage_range(&self) -> DamageRange {
        let (min, max) = match self.rarity {
            MonsterRarity::Normal => (1., 2.),
            MonsterRarity::Rare => (2., 4.),
        };
        let multiplier = (self.level + 1) as f32;
        let min = min * multiplier;
        let max = max * multiplier;
        DamageRange(min..=max)
    }

    pub fn xp(&self) -> XpOnDeath {
        let xp = match self.rarity {
            MonsterRarity::Normal => 1u32,
            MonsterRarity::Rare => 3,
        };
        XpOnDeath(xp * (self.level + 1) as u32)
    }

    pub fn movement_speed(&self) -> BaseMovementSpeed {
        match self.rarity {
            MonsterRarity::Normal => BaseMovementSpeed(90.),
            MonsterRarity::Rare => BaseMovementSpeed(70.),
        }
    }

    pub fn life(&self) -> BaseLife {
        let life = match self.rarity {
            MonsterRarity::Normal => 2.,
            MonsterRarity::Rare => 5.,
        };
        // 5% increase life per level
        let incr = self.level as f32 * 5.;
        BaseLife(life * (100. + incr) / 100.)
    }
}

#[derive(Component, Deref)]
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

impl MonsterSpawnConfig {
    pub fn new(params: MonsterSpawnParams) -> Self {
        MonsterSpawnConfig {
            timer: Timer::from_seconds(1., TimerMode::Once),
            params,
        }
    }
}

#[derive(Component)]
pub struct MonsterFuturePos;

#[derive(Bundle)]
pub struct MonsterFuturePosBundle {
    tag: MonsterFuturePos,
    name: Name,
    sprite: Sprite,
    transform: Transform,
    config: MonsterSpawnConfig,
}

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
