use crate::in_game::collisions::GROUP_BONUS;
use crate::prelude::*;
use rand::{thread_rng, Rng};

pub struct BonusPlugin;

impl Plugin for BonusPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_asset).add_system(spawn_bonus);
    }
}

const MONEY_ASSET_PATH: &str = "items/crystal_01a.png";

const BONUS_SIZE: Vec2 = Vec2::new(0.8, 0.8);

fn load_asset(asset_server: Res<AssetServer>, mut textures: ResMut<GameTextures>) {
    textures.money = asset_server.load(MONEY_ASSET_PATH);
}

fn spawn_bonus(
    mut commands: Commands,
    mut monster_death_events: EventReader<MonsterDeathEvent>,
    textures: Res<GameTextures>,
) {
    let mut rng = thread_rng();
    for event in monster_death_events.iter() {
        if rng.gen_range(0..100) < 20 {
            commands
                .spawn(Bonus)
                .insert(Name::new("Bonus"))
                // sprite
                .insert(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(BONUS_SIZE),
                        ..Default::default()
                    },
                    texture: textures.money.clone(),
                    transform: Transform::from_translation(event.pos),
                    ..Default::default()
                })
                // rapier
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(BONUS_SIZE.x / 2.0, BONUS_SIZE.y / 2.0))
                .insert(CollisionGroups::new(GROUP_BONUS, Group::ALL));
        }
    }
}
