use super::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Bonus;

#[derive(Bundle)]
pub struct BonusBundle {
    tag: Bonus,
    name: Name,
    sprite: SpriteBundle,
    body: RigidBody,
    collider: Collider,
    collision_groups: CollisionGroups,
}

impl BonusBundle {
    pub fn new(pos: Vec3, assets: &BonusAssets) -> Self {
        BonusBundle {
            tag: Bonus,
            name: Name::new("Bonus"),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(BONUS_SIZE),
                    ..Default::default()
                },
                texture: assets.texture.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(BONUS_SIZE.x / 2.0, BONUS_SIZE.y / 2.0),
            collision_groups: CollisionGroups::new(GROUP_BONUS, Group::ALL),
        }
    }
}

const BONUS_SIZE: Vec2 = Vec2::new(12., 12.);

#[derive(Resource)]
pub struct BonusAssets {
    pub texture: Handle<Image>,
}
