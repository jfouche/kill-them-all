use super::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    tag: Player,
    name: Name,
    money: Money,
    xp: Experience,
    // bevy view
    sprite: SpriteBundle,
    texture_atlas: TextureAtlas,
    animation_timer: AnimationTimer,
    // skills
    skills: SkillsBundle,
    weapon: Weapon,
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
            money: Money(0),
            xp: Experience::default(),
            sprite: SpriteBundle::default(),
            texture_atlas: TextureAtlas::default(),
            animation_timer: AnimationTimer::default(),
            skills: SkillsBundle {
                life: Life::new(10),
                movement_speed: MovementSpeed::new(130.),
                ..Default::default()
            },
            weapon: WeaponType::Gun.into(),
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
}

impl PlayerHitEvent {
    pub fn new(entity: Entity) -> Self {
        PlayerHitEvent { entity }
    }
}

/// Event to notify the player died
#[derive(Event)]
pub struct PlayerDeathEvent;

/// Event to notify a player level up
#[derive(Event)]
pub struct LevelUpEvent;
