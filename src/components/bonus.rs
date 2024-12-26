use super::*;
use crate::utils::despawn_after::DespawnAfter;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
#[require(
    Name(|| Name::new("Bonus")),
    Sprite,
    RigidBody(|| RigidBody::Fixed),
    Collider(|| Collider::cuboid(BONUS_SIZE.x / 2.0, BONUS_SIZE.y / 2.0)),
    CollisionGroups(|| CollisionGroups::new(GROUP_BONUS, GROUP_ALL)),
    DespawnAfter(despawn_after)
)]
pub struct Bonus;

impl Bonus {
    pub fn sprite(assets: &BonusAssets) -> Sprite {
        Sprite {
            image: assets.texture.clone(),
            custom_size: Some(BONUS_SIZE),
            ..Default::default()
        }
    }
}

fn despawn_after() -> DespawnAfter {
    DespawnAfter::new(Duration::from_secs(8)).with_blink(Duration::from_secs(3))
}

const BONUS_SIZE: Vec2 = Vec2::new(12., 12.);

#[derive(Resource)]
pub struct BonusAssets {
    pub texture: Handle<Image>,
}

impl FromWorld for BonusAssets {
    fn from_world(world: &mut World) -> Self {
        BonusAssets {
            texture: world.load_asset("items/crystal_01a.png"),
        }
    }
}
