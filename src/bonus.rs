use crate::prelude::*;
use rand::{thread_rng, Rng};

pub struct BonusPlugin;

impl Plugin for BonusPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_bonus);
    }
}

fn spawn_bonus(mut commands: Commands, mut monster_death_events: EventReader<MonsterDeathEvent>) {
    let mut rng = thread_rng();
    for event in monster_death_events.iter() {
        if rng.gen_range(0..100) < 20 {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(0.4, 0.4)),
                    ..Default::default()
                },
                transform: Transform::from_translation(event.pos),
                ..Default::default()
            });
        }
    }
}
