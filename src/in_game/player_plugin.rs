use crate::components::*;
use crate::schedule::*;
use crate::utils::blink::Blink;
use crate::utils::invulnerable::Invulnerable;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::ops::Mul;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDeathEvent>()
            .register_type::<Experience>()
            .add_systems(Startup, load_player_assets)
            .add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(OnExit(GameState::InGame), despawn_all::<Player>)
            .add_systems(OnEnter(GameState::InGame), unpause)
            .add_systems(OnExit(GameState::InGame), pause)
            .add_systems(
                Update,
                (
                    player_movement,
                    animate_player_sprite,
                    player_fires,
                    player_invulnerability_finished,
                    increment_player_experience,
                    level_up,
                    // on_player_hit,
                )
                    .in_set(GameRunningSet::EntityUpdate),
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

fn load_player_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("characters/RedNinja/SpriteSheet.png");
    let texture_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 4, 7, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas_layout);

    let player_assets = PlayerAssets {
        texture: texture_handle,
        texture_atlas_layout: texture_atlas_handle,
    };
    commands.insert_resource(player_assets)
}

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands
        .spawn(PlayerBundle::from_assets(&assets))
        .observe(trigger_player_hit);
}

fn pause(mut query: Query<(&mut Invulnerable, &mut Blink), With<Player>>) {
    if let Ok((mut invulnerable, mut blink)) = query.get_single_mut() {
        invulnerable.pause(true);
        blink.pause(true);
    }
}

fn unpause(mut query: Query<(&mut Invulnerable, &mut Blink), With<Player>>) {
    if let Ok((mut invulnerable, mut blink)) = query.get_single_mut() {
        invulnerable.pause(false);
        blink.pause(false);
    }
}

///
/// Manage the keyboard to move the player
///
fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&MovementSpeed, &mut Velocity), With<Player>>,
) {
    if let Ok((speed, mut velocity)) = players.get_single_mut() {
        let mut linvel = Vec2::default();
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::Numpad4, KeyCode::KeyA]) {
            linvel.x = -1.0;
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::Numpad6, KeyCode::KeyD]) {
            linvel.x = 1.0;
        }
        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::Numpad8, KeyCode::KeyW]) {
            linvel.y = 1.0;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::Numpad2, KeyCode::KeyS]) {
            linvel.y = -1.0;
        }
        velocity.linvel = linvel.normalize_or_zero().mul(**speed);
    }
}

///
/// Player fires
///
fn player_fires(
    mut commands: Commands,
    time: Res<Time>,
    mut q_player: Query<
        (&Transform, &mut Weapon, &IncreaseAttackSpeed, &PierceChance),
        With<Player>,
    >,
    q_monsters: Query<&Transform, With<Monster>>,
) {
    if let Ok((player, mut weapon, attack_speed, pierce)) = q_player.get_single_mut() {
        weapon.tick(time.delta(), **attack_speed);
        if weapon.ready() {
            let player = player.translation;
            // Get the nearest monster
            let nearest_monster = q_monsters
                .iter()
                .map(|transform| transform.translation)
                .reduce(|nearest, other| {
                    if player.distance(other) < player.distance(nearest) {
                        other // new nearest
                    } else {
                        nearest
                    }
                });
            if let Some(nearest) = nearest_monster {
                commands.spawn(BulletBundle::new(BulletOptions::new(
                    player,
                    PLAYER_SIZE,
                    weapon.attack(),
                    **pierce,
                    nearest,
                )));
            }
        }
    }
}

///
/// player hit
///
fn trigger_player_hit(
    hit_event: Trigger<HitEvent>,
    mut commands: Commands,
    mut q_player: Query<(&Equipments, &mut Life, &mut CollisionGroups), With<Player>>,
    mut send_death: EventWriter<PlayerDeathEvent>,
) {
    let player_entity = hit_event.entity();
    if let Ok((equipments, mut life, mut collision_groups)) = q_player.get_mut(player_entity) {
        let damage = hit_event.event().damage - equipments.armour();
        info!("on_player_hit: damage: {:.0}", *damage);
        if *damage > 0. {
            life.hit(damage);
            if life.is_dead() {
                commands.entity(player_entity).despawn();
                send_death.send(PlayerDeathEvent);
            } else {
                // Set player invulnerable
                commands
                    .entity(player_entity)
                    .insert(Invulnerable::new(Duration::from_secs_f32(2.0), GROUP_ENEMY))
                    .insert(Blink::new(Duration::from_secs_f32(0.15)));

                // To allow player to not collide with enemies
                collision_groups.filters &= !GROUP_ENEMY;
            }
        }
    }
}

///
/// Animate the player sprite
///
fn animate_player_sprite(
    time: Res<Time>,
    mut q_player: Query<(&Velocity, &mut AnimationTimer, &mut TextureAtlas), With<Player>>,
) {
    if let Ok((&velocity, mut timer, mut atlas)) = q_player.get_single_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if velocity == Velocity::zero() {
                0
            } else {
                match atlas.index {
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

fn player_invulnerability_finished(
    mut commands: Commands,
    q_player: Query<(), With<Player>>,
    mut entities: RemovedComponents<Invulnerable>,
) {
    for entity in entities.read() {
        if q_player.get(entity).is_ok() {
            info!("player_invulnerability_finished");
            commands.entity(entity).remove::<Blink>();
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
        for monster_death_ev in monster_death_reader.read() {
            info!("increment_player_experience");
            let level_before = experience.level();
            experience.add(monster_death_ev.xp);
            if experience.level() > level_before {
                // LEVEL UP !
                level_up_sender.send(LevelUpEvent);
            }
        }
    }
}

fn level_up(
    mut q_player: Query<(&mut Life, &MaxLife), With<Player>>,
    mut level_up_rcv: EventReader<LevelUpEvent>,
) {
    if let Ok((mut life, max_life)) = q_player.get_single_mut() {
        for _ in level_up_rcv.read() {
            info!("level_up");
            // Regen life
            life.regenerate(**max_life);
        }
    }
}
