mod gun;
pub use gun::*;

use crate::components::*;
use bevy::{app::PluginGroupBuilder, prelude::*};

use super::game_is_running;

pub struct WeaponsPluginGroup;

impl PluginGroup for WeaponsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(gun::GunPlugin)
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
            (update_weapon_timer_duration, tick_weapon)
                .chain()
                .run_if(game_is_running),
        );
}

fn update_weapon_timer_duration(
    mut weapons: Query<(&mut AttackTimer, &BaseAttackSpeed, &Parent), With<Weapon>>,
    characters: Query<&IncreaseAttackSpeed>,
) {
    for (mut timer, base_attack_speed, parent) in &mut weapons {
        if let Ok(increase) = characters.get(**parent) {
            let attack_speed = base_attack_speed * increase;
            timer.set_attack_speed(attack_speed);
        }
    }
}

fn tick_weapon(mut weapons: Query<&mut AttackTimer, With<Weapon>>, time: Res<Time>) {
    for mut timer in &mut weapons {
        timer.tick(time.delta());
    }
}
