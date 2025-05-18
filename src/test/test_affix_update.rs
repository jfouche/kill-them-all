use crate::assert_approx_eq;
use crate::components::item::ItemSpawnConfig;
use crate::components::skills::shuriken::ShurikenLauncher;
use crate::components::{
    affix::{
        Armour, BaseArmour, IncreaseArmour, IncreaseAttackSpeed, IncreaseDamage, MoreArmour,
        MoreDamage,
    },
    character::Character,
    damage::HitDamageRange,
    equipment::{
        weapon::{AttackSpeed, AttackTimer},
        Helmet, Wand,
    },
    skills::fireball::FireBallLauncher,
};
use crate::in_game::affix_updates_plugin::AffixUpdatesPlugin;
use crate::schedule::{GameState, InGameState};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, AffixUpdatesPlugin))
        .insert_state(GameState::InGame)
        .insert_state(InGameState::Running);
    app
}

#[test]
fn test_update_equipment_armour() {
    let mut app = create_app();

    let helmet = app
        .world_mut()
        .spawn((
            Helmet::new(1),
            BaseArmour(1.),
            MoreArmour(3.),
            IncreaseArmour(50.),
        ))
        .id();

    app.update();
    println!("YOUHOU");

    let armour = app.world().get::<Armour>(helmet);
    assert_eq!(6., armour.unwrap().0);
}

#[test]
fn test_update_character_armour() {
    let mut app = create_app();

    let character = app
        .world_mut()
        .spawn(Character)
        .with_children(|parent| {
            parent.spawn((
                Helmet::new(1),
                BaseArmour(1.),
                MoreArmour(3.),
                IncreaseArmour(50.),
            ));
            parent.spawn(MoreArmour(4.));
            parent.spawn(MoreArmour(10.));
        })
        .id();

    app.update();

    let armour = app.world().get::<Armour>(character);
    assert_eq!(20., armour.unwrap().0);
}

#[test]
fn test_skill_attack_speed() {
    let mut app = create_app();

    let skill_alone = app.world_mut().spawn(FireBallLauncher).id();
    app.world_mut()
        .spawn(Character)
        .with_children(|parent| {
            parent.spawn(Wand::new(1));
        })
        .add_child(skill_alone);

    let skill_with_affixes = app.world_mut().spawn(ShurikenLauncher).id();
    app.world_mut()
        .spawn(Character)
        .with_children(|parent| {
            parent.spawn((Wand::new(1), IncreaseAttackSpeed(50.)));
            parent.spawn(IncreaseAttackSpeed(20.));
        })
        .add_child(skill_with_affixes);

    app.update();

    let attack_speed = app.world().get::<AttackSpeed>(skill_alone);
    assert_eq!(1.0, attack_speed.unwrap().0);
    let attack_timer = app.world().get::<AttackTimer>(skill_alone);
    assert_eq!(1. / 1.0, attack_timer.unwrap().duration().as_secs_f32());

    // 0.6 + 50% + 20%
    let attack_speed = app.world().get::<AttackSpeed>(skill_with_affixes);
    assert_approx_eq!(1.08, attack_speed.unwrap().0);
    let attack_timer = app.world().get::<AttackTimer>(skill_with_affixes);
    assert_eq!(1. / 1.08, attack_timer.unwrap().duration().as_secs_f32());
}

#[test]
fn test_skill_damage_range() {
    let mut app = create_app();

    let skill_alone = app.world_mut().spawn(FireBallLauncher).id();
    app.world_mut()
        .spawn(Character)
        // .with_children(|parent| {
        //     parent.spawn(Wand);
        // })
        .add_child(skill_alone);

    let skill_with_affixes = app.world_mut().spawn(FireBallLauncher).id();
    app.world_mut()
        .spawn(Character)
        .with_children(|parent| {
            parent.spawn((Wand::new(1), MoreDamage(5.), IncreaseDamage(50.)));
            parent.spawn(MoreDamage(2.));
            parent.spawn(IncreaseDamage(10.));
        })
        .add_child(skill_with_affixes);

    app.update();

    let damage_range = app.world().get::<HitDamageRange>(skill_alone);
    assert_approx_eq!(1., damage_range.unwrap().min);
    assert_approx_eq!(2., damage_range.unwrap().max);

    let damage_range = app.world().get::<HitDamageRange>(skill_with_affixes);
    // weapon : (1..2 + 5 ) * 50% = 9..10.5
    // skill : (1..2 + 9..10.5 + 2) * 10% = 13.2..15.95
    assert_approx_eq!(13.2, damage_range.unwrap().min);
    assert_approx_eq!(15.95, damage_range.unwrap().max);
}
