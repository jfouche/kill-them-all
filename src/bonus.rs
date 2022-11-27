use crate::prelude::*;
use rand::{thread_rng, Rng};

pub struct BonusPlugin;

impl Plugin for BonusPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_asset).add_system(spawn_bonus);
    }
}

const MONEY_ASSET_PATH: &str = "items/crystal_01a.png";

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
                .insert(SpriteBundle {
                    texture: textures.money.clone(),
                    transform: Transform::from_translation(event.pos)
                        .with_scale(Vec3::new(0.02, 0.02, 1.)),
                    ..Default::default()
                });
        }
    }
}
