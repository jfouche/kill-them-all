mod gun;
pub use gun::*;

mod shuriken;
pub use shuriken::*;

use super::game_is_running;
use crate::components::*;
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct WeaponsPluginGroup;

impl PluginGroup for WeaponsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(gun::GunPlugin)
            .add(shuriken::ShurikenPlugin)
            .add(weapons_plugin)
    }
}

fn weapons_plugin(app: &mut App) {
    app.register_type::<Damage>()
        .register_type::<BaseAttackSpeed>()
        .register_type::<AttackSpeed>()
        .register_type::<DamageRange>()
        .register_type::<AttackTimer>()
        .add_systems(
            PreUpdate,
            (update_weapon_attack_speed, tick_weapon)
                .chain()
                .run_if(game_is_running),
        );
}

fn update_weapon_attack_speed(
    mut weapons: Query<
        (
            &mut AttackTimer,
            &mut AttackSpeed,
            &BaseAttackSpeed,
            &Parent,
        ),
        With<Weapon>,
    >,
    characters: Query<&IncreaseAttackSpeed>,
) {
    for (mut timer, mut attack_speed, base_attack_speed, parent) in &mut weapons {
        if let Ok(increase) = characters.get(**parent) {
            *attack_speed = base_attack_speed * increase;
            timer.set_attack_speed(*attack_speed);
        }
    }
}

fn tick_weapon(mut weapons: Query<&mut AttackTimer, With<Weapon>>, time: Res<Time>) {
    for mut timer in &mut weapons {
        timer.tick(time.delta());
    }
}
