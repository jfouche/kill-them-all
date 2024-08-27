use super::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

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
    sprite: SpriteBundle,
    texture_atlas: TextureAtlas,
    animation_timer: AnimationTimer,
    // skills
    skills: SkillsBundle,
    xp_on_death: XpOnDeath,
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
            sprite: SpriteBundle::default(),
            texture_atlas: TextureAtlas::default(),
            animation_timer: AnimationTimer::default(),
            skills: SkillsBundle {
                life: Life::new(20),
                movement_speed: MovementSpeed::new(80.),
                ..Default::default()
            },
            xp_on_death: XpOnDeath(1),
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
            skills: params.into(),
            xp_on_death: params.into(),
            sprite: SpriteBundle {
                texture: assets.texture.clone(),
                sprite: Sprite {
                    custom_size: Some(size),
                    ..Default::default()
                },
                transform: Transform::from_xyz(params.pos.x, params.pos.y, 10.),
                ..Default::default()
            },
            texture_atlas: TextureAtlas {
                layout: assets.texture_atlas_layout.clone(),
                ..Default::default()
            },
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
}

impl MonsterSpawnParams {
    pub fn rand() -> Self {
        let mut rng = thread_rng();
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
        }
    }

    fn size(&self) -> Vec2 {
        match self.rarity {
            MonsterRarity::Normal => Vec2::new(16.0, 16.0),
            MonsterRarity::Rare => Vec2::new(32.0, 32.0),
        }
    }
}

impl From<&MonsterSpawnParams> for SkillsBundle {
    fn from(value: &MonsterSpawnParams) -> Self {
        match value.rarity {
            MonsterRarity::Normal => SkillsBundle {
                movement_speed: MovementSpeed::new(80.),
                life: Life::new(2),
                ..Default::default()
            },
            MonsterRarity::Rare => SkillsBundle {
                movement_speed: MovementSpeed::new(70.),
                life: Life::new(5),
                ..Default::default()
            },
        }
    }
}

#[derive(Component, Deref)]
pub struct XpOnDeath(pub u32);

impl From<&MonsterSpawnParams> for XpOnDeath {
    fn from(value: &MonsterSpawnParams) -> Self {
        let xp = match value.rarity {
            MonsterRarity::Normal => 1,
            MonsterRarity::Rare => 3,
        };
        XpOnDeath(xp)
    }
}

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
    sprite: SpriteBundle,
    config: MonsterSpawnConfig,
}

impl MonsterFuturePosBundle {
    pub fn new(params: MonsterSpawnParams) -> Self {
        MonsterFuturePosBundle {
            tag: MonsterFuturePos,
            name: Name::new("MonsterFuturePos"),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.8, 0.3, 0.3, 0.2),
                    custom_size: Some(params.size()),
                    ..Default::default()
                },
                transform: Transform::from_xyz(params.pos.x, params.pos.y, 1.),
                ..Default::default()
            },
            config: MonsterSpawnConfig::new(params),
        }
    }
}

/// Event to notify a monster was hit
#[derive(Event)]
pub struct MonsterHitEvent {
    pub entity: Entity,
    pub damage: u16,
}

impl MonsterHitEvent {
    pub fn new(entity: Entity, damage: u16) -> Self {
        MonsterHitEvent { entity, damage }
    }
}

/// Event to notify a monster died
#[derive(Event)]
pub struct MonsterDeathEvent {
    pub entity: Entity,
    pub pos: Vec3,
    pub xp: u32,
}
