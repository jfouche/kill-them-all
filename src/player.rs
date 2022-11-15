use std::ops::Mul;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    bullets::{spawn_bullet_at, BulletOptions},
    components::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerHitEvent>()
            .add_startup_system(setup)
            .add_system(player_movement)
            .add_system(animate_sprite.after(player_movement))
            .add_system(player_fires)
            .add_system(on_player_hit);
    }
}

struct PlayerFireConfig {
    /// timer between attacks per seconds
    timer: Timer,
}

pub struct PlayerHitEvent {
    entity: Entity,
}

impl PlayerHitEvent {
    pub fn new(entity: Entity) -> Self {
        PlayerHitEvent { entity }
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

impl AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.1, true))
    }
}

const PLAYER_SIZE: Vec2 = Vec2::new(1.0, 1.0);

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    sprite: SpriteSheetBundle,
    player: Player,
    speed: Speed,
    life: Life,
    body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    constraints: LockedAxes,
    gravity: GravityScale,
    events: ActiveEvents,
    animation_timer: AnimationTimer,
}

impl PlayerBundle {
    fn new(texture_atlas_handle: Handle<TextureAtlas>) -> Self {
        PlayerBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_xyz(0., 0., 10.).with_scale(Vec3::splat(1.0 / 16.0)),
                ..Default::default()
            },
            player: Player,
            speed: Speed(8.),
            life: Life::new(10),
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(PLAYER_SIZE.x / 2., PLAYER_SIZE.y / 2.),
            gravity: GravityScale(0.0),
            constraints: LockedAxes::ROTATION_LOCKED,
            events: ActiveEvents::COLLISION_EVENTS,
            velocity: Velocity::default(),
            animation_timer: AnimationTimer::default(),
        }
    }
}

///
///  
///
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // load player texture_atlas
    let texture_handle = asset_server.load("characters/RedNinja/SpriteSheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 7);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // spawn player
    commands
        .spawn_bundle(PlayerBundle::new(texture_atlas_handle))
        .insert(Name::new("Player"));

    commands.insert_resource(PlayerFireConfig {
        timer: Timer::from_seconds(1., true),
    });
}

///
/// Manage the keyboard to move the player
///
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Speed, &mut Velocity), With<Player>>,
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
        velocity.linvel = linvel.normalize_or_zero().mul(**speed);
    }
}

///
/// Spawn monster at Timer times
///
fn player_fires(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<PlayerFireConfig>,
    q_player: Query<&Transform, With<Player>>,
    q_monsters: Query<&Transform, With<Monster>>,
) {
    // tick the timer
    config.timer.tick(time.delta());

    if config.timer.finished() {
        if let Ok(player) = q_player.get_single() {
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
                spawn_bullet_at(
                    &mut commands,
                    BulletOptions::new(player, PLAYER_SIZE, nearest),
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
    mut q_player: Query<&mut Life, With<Player>>,
    // mut send_player_death: EventWriter<PlayerDeathEvent>,
) {
    if let Ok(mut life) = q_player.get_single_mut() {
        for event in player_hit_events.iter() {
            warn!("on_player_hit");
            life.hit(1);
            if life.is_dead() {
                commands.entity(event.entity).despawn();
            }
        }
    }
}

///
/// Animate the player sprite
///
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
    q_player: Query<&Velocity, With<Player>>,
) {
    if let Ok(&velocity) = q_player.get_single() {
        for (mut timer, mut sprite) in &mut query {
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
}
