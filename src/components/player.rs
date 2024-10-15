use super::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    tag: Player,
    name: Name,
    // Equipments
    equipments: Equipments,
    //
    money: Money,
    xp: Experience,
    // bevy view
    sprite: SpriteBundle,
    texture_atlas: TextureAtlas,
    animation_timer: AnimationTimer,
    // skills
    skills: SkillsBundle,
    weapon: Weapon,
    upgrades: Upgrades,
    // physics
    body: RigidBody,
    velocity: Velocity,
    collider: Collider,
    collision_groups: CollisionGroups,
    locked_axes: LockedAxes,
    active_envents: ActiveEvents,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            tag: Player,
            name: Name::new("Player"),
            equipments: Equipments::default(),
            money: Money(0),
            xp: Experience::default(),
            sprite: SpriteBundle::default(),
            texture_atlas: TextureAtlas::default(),
            animation_timer: AnimationTimer::default(),
            skills: SkillsBundle {
                life: LifeBundle::new(10.),
                movement_speed: MovementSpeedBundle::new(130.),
                ..Default::default()
            },
            weapon: WeaponType::Gun.into(),
            upgrades: Upgrades::default(),
            body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            collider: Collider::cuboid(PLAYER_SIZE.x / 2., PLAYER_SIZE.y / 2.),
            collision_groups: CollisionGroups::new(GROUP_PLAYER, Group::ALL),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            active_envents: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

impl PlayerBundle {
    pub fn from_assets(assets: &PlayerAssets) -> Self {
        PlayerBundle {
            sprite: SpriteBundle {
                texture: assets.texture.clone(),
                sprite: Sprite {
                    custom_size: Some(PLAYER_SIZE),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., 0., 10.),
                ..Default::default()
            },
            texture_atlas: TextureAtlas {
                layout: assets.texture_atlas_layout.clone(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub const PLAYER_SIZE: Vec2 = Vec2::new(16.0, 16.0);

#[derive(Resource)]
pub struct PlayerAssets {
    pub texture: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

/// Event to notify the player was hit
#[derive(Event)]
pub struct PlayerHitEvent {
    pub entity: Entity,
    pub damage: Damage,
}

impl PlayerHitEvent {
    pub fn new(entity: Entity, damage: Damage) -> Self {
        PlayerHitEvent { entity, damage }
    }
}

/// Event to notify the player died
#[derive(Event)]
pub struct PlayerDeathEvent;

/// Event to notify a player level up
#[derive(Event)]
pub struct LevelUpEvent;

// ==================================================================
// Experience

#[derive(Component, Default, Reflect)]
pub struct Experience(u32);

impl Experience {
    const LEVELS: [u32; 6] = [4, 10, 30, 80, 170, 300];

    pub fn add(&mut self, xp: u32) {
        self.0 += xp;
    }

    pub fn current(&self) -> u32 {
        self.0
    }

    /// Level starting at 0
    pub fn level(&self) -> u8 {
        let mut level = 0;
        for xp in Experience::LEVELS.iter() {
            if self.0 >= *xp {
                level += 1;
            } else {
                break;
            }
        }
        level
    }

    pub fn get_current_level_min_max_exp(&self) -> (u32, u32) {
        let level = self.level();
        let min = match level {
            0 => &0,
            _ => Experience::LEVELS.get(level as usize - 1).unwrap_or(&100),
        };
        let max = Experience::LEVELS
            .get(level as usize)
            .unwrap_or(Experience::LEVELS.last().unwrap());
        (*min, *max)
    }
}

impl std::fmt::Display for Experience {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{} (level {})",
            self.0,
            self.get_current_level_min_max_exp().1,
            self.level() + 1,
        )
    }
}
