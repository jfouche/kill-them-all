use crate::in_game::bullets::{spawn_bullet_at, BulletOptions};
use crate::in_game::collisions::GROUP_ENEMY;
use crate::prelude::invulnerable::Invulnerable;
use crate::prelude::*;
use std::ops::Mul;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_movement)
                .with_system(animate_sprite)
                .with_system(player_fires)
                .with_system(on_player_hit)
                .with_system(increment_player_experience)
                .with_system(level_up),
        );
    }
}

#[derive(Component, Deref, DerefMut)]
struct InvulnerabilityAnimationTimer(Timer);

impl Default for InvulnerabilityAnimationTimer {
    fn default() -> Self {
        InvulnerabilityAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

const PLAYER_SIZE: Vec2 = Vec2::new(1.0, 1.0);

///
/// Manage the keyboard to move the player
///
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&MovementSpeed, &mut Velocity), With<Player>>,
) {
    for (speed, mut velocity) in players.iter_mut() {
        let mut linvel = Vec2::default();
        if keyboard_input.any_pressed([KeyCode::Left, KeyCode::Numpad4]) {
            linvel.x = -1.0;
        }
        if keyboard_input.any_pressed([KeyCode::Right, KeyCode::Numpad6]) {
            linvel.x = 1.0;
        }
        if keyboard_input.any_pressed([KeyCode::Up, KeyCode::Numpad8]) {
            linvel.y = 1.0;
        }
        if keyboard_input.any_pressed([KeyCode::Down, KeyCode::Numpad2]) {
            linvel.y = -1.0;
        }
        velocity.linvel = linvel.normalize_or_zero().mul(speed.value());
    }
}

///
/// Player fires
///
fn player_fires(
    mut commands: Commands,
    time: Res<Time>,
    mut q_player: Query<(&Transform, &mut Weapon, &AttackSpeed), With<Player>>,
    q_monsters: Query<&Transform, With<Monster>>,
) {
    if let Ok((player, mut weapon, attack_speed)) = q_player.get_single_mut() {
        weapon.tick(time.delta(), attack_speed.value());
        if weapon.ready() {
            let player = player.translation;
            // Get the nearest monster
            let nearest_monster = q_monsters
                .iter()
                .map(|transform| transform.translation)
                .reduce(|current, other| {
                    if player.distance(other) < player.distance(current) {
                        other
                    } else {
                        current
                    }
                });
            if let Some(nearest) = nearest_monster {
                let damage = weapon.attack();
                spawn_bullet_at(
                    &mut commands,
                    BulletOptions::new(player, damage, PLAYER_SIZE, nearest),
                );
            }
        }
    }
}

///
/// player hit
///
fn on_player_hit(
    mut commands: Commands,
    mut player_hit_events: EventReader<PlayerHitEvent>,
    mut q_player: Query<(&mut Life, &mut CollisionGroups), With<Player>>,
    mut send_death: EventWriter<PlayerDeathEvent>,
) {
    if let Ok((mut life, mut collision_groups)) = q_player.get_single_mut() {
        for event in player_hit_events.iter() {
            info!("on_player_hit");
            life.hit(1);
            if life.is_dead() {
                commands.entity(event.entity).despawn();
                send_death.send(PlayerDeathEvent);
                // break to ensure we don't try to despawn player if already dead
                break;
            } else {
                // Set player invulnerable
                commands
                    .entity(event.entity)
                    .insert(Invulnerable::new(Duration::from_secs_f32(2.0), GROUP_ENEMY));

                // To allow player to not collide with enemies
                collision_groups.filters &= !GROUP_ENEMY;
            }
        }
    }
}

///
/// Animate the player sprite
///
fn animate_sprite(
    time: Res<Time>,
    mut q_player: Query<(&Velocity, &mut AnimationTimer, &mut TextureAtlasSprite), With<Player>>,
) {
    if let Ok((&velocity, mut timer, mut sprite)) = q_player.get_single_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if velocity == Velocity::zero() {
                0
            } else {
                match sprite.index {
                    4 => 8,
                    8 => 12,
                    12 => 16,
                    16 => 4,
                    _ => 4,
                }
            }
        }
    }
}

///
/// Update player XP when monster died
///
fn increment_player_experience(
    mut monster_death_reader: EventReader<MonsterDeathEvent>,
    mut q_player: Query<&mut Experience, With<Player>>,
    mut level_up_sender: EventWriter<LevelUpEvent>,
) {
    if let Ok(mut experience) = q_player.get_single_mut() {
        for _ in monster_death_reader.iter() {
            info!("increment_player_experience");
            let level_before = experience.level();
            experience.add(1);
            if experience.level() > level_before {
                // LEVEL UP !
                level_up_sender.send(LevelUpEvent);
            }
        }
    }
}

fn level_up(
    mut q_player: Query<&mut Life, With<Player>>,
    mut level_up_rcv: EventReader<LevelUpEvent>,
) {
    if let Ok(mut life) = q_player.get_single_mut() {
        for _ in level_up_rcv.iter() {
            info!("level_up");
            // Regen life
            let max_life = life.max_life();
            life.regenerate(max_life);
        }
    }
}
