use crate::components::*;
use crate::in_game::AffixUpdatesPlugin;
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
        .spawn((Helmet, BaseArmour(1.), MoreArmour(3.), IncreaseArmour(50.)))
        .id();

    app.update();

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
            parent.spawn((Helmet, BaseArmour(1.), MoreArmour(3.), IncreaseArmour(50.)));
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
            parent.spawn(Wand);
        })
        .add_child(skill_alone);

    let skill_with_affixes = app.world_mut().spawn(FireBallLauncher).id();
    app.world_mut()
        .spawn(Character)
        .with_children(|parent| {
            parent.spawn((Wand, IncreaseAttackSpeed(50.)));
            parent.spawn(IncreaseAttackSpeed(20.));
        })
        .add_child(skill_with_affixes);

    app.update();

    let attack_speed = app.world().get::<AttackSpeed>(skill_alone);
    assert_eq!(1.2, attack_speed.unwrap().0);
    let attack_timer = app.world().get::<AttackTimer>(skill_alone);
    assert_eq!(1. / 1.2, attack_timer.unwrap().duration().as_secs_f32());

    let attack_speed = app.world().get::<AttackSpeed>(skill_with_affixes);
    assert_eq!(2.16, attack_speed.unwrap().0);
    let attack_timer = app.world().get::<AttackTimer>(skill_with_affixes);
    assert_eq!(1. / 2.16, attack_timer.unwrap().duration().as_secs_f32());
}

#[test]
fn test_skill_damage_range() {
    let mut app = create_app();

    let skill_alone = app.world_mut().spawn(FireBallLauncher).id();
    app.world_mut()
        .spawn(Character)
        .with_children(|parent| {
            parent.spawn(Wand);
        })
        .add_child(skill_alone);

    let skill_with_affixes = app.world_mut().spawn(FireBallLauncher).id();
    app.world_mut()
        .spawn(Character)
        .with_children(|parent| {
            parent.spawn((Wand, MoreDamage(5.), IncreaseDamage(50.)));
            parent.spawn(MoreDamage(2.));
            parent.spawn(IncreaseDamage(10.));
        })
        .add_child(skill_with_affixes);

    app.update();

    let damage_range = app.world().get::<HitDamageRange>(skill_alone);
    assert_eq!(1., damage_range.unwrap().min);
    assert_eq!(2., damage_range.unwrap().max);

    let damage_range = app.world().get::<HitDamageRange>(skill_with_affixes);
    assert_eq!(12.1, damage_range.unwrap().min);
    assert_eq!(13.75, damage_range.unwrap().max);
}
