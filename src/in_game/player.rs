use crate::in_game::bullets::{spawn_bullet_at, BulletOptions};
use crate::in_game::collisions::{GROUP_ENEMY, GROUP_PLAYER};
use crate::prelude::invulnerable::Invulnerable;
use crate::prelude::*;
use std::ops::Mul;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    player_movement,
                    animate_sprite,
                    player_fires,
                    on_player_hit,
                    player_invulnerability_finished,
                    increment_player_experience,
                    level_up,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnEnter(GameState::InGame), unpause)
            .add_systems(OnExit(GameState::InGame), pause);
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

fn spawn_player(
    commands: &mut Commands,
    config: PlayerConfig,
    texture: Handle<Image>,
    texture_atlas_handle: Handle<TextureAtlasLayout>,
) {
    commands
        .spawn(Player)
        .insert(Name::new("Player"))
        .insert(MovementSpeed::new(config.movement_speed))
        .insert(Life::new(config.life))
        .insert(AttackSpeed::new())
        .insert(Weapon::new(config.attack_speed, 1, 4))
        .insert(Money(0))
        .insert(Experience::default())
        // Sprite
        .insert(SpriteBundle {
            texture,
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 10.),
            ..Default::default()
        })
        .insert(TextureAtlas {
            layout: texture_atlas_handle,
            ..Default::default()
        })
        .insert(AnimationTimer::default())
        // Rapier
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(PLAYER_SIZE.x / 2., PLAYER_SIZE.y / 2.))
        .insert(CollisionGroups::new(GROUP_PLAYER, Group::ALL))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::default());
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // load player texture_atlas
    let texture_handle = asset_server.load("characters/RedNinja/SpriteSheet.png");
    let texture_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 4, 7, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas_layout);

    let player_config = PlayerConfig {
        life: 20,
        movement_speed: 8.,
        attack_speed: 1.,
    };
    spawn_player(
        &mut commands,
        player_config,
        texture_handle,
        texture_atlas_handle,
    );
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
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::Numpad4]) {
            linvel.x = -1.0;
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::Numpad6]) {
            linvel.x = 1.0;
        }
        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::Numpad8]) {
            linvel.y = 1.0;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::Numpad2]) {
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
        for event in player_hit_events.read() {
            warn!("on_player_hit");
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
fn animate_sprite(
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
        for _ in monster_death_reader.read() {
            warn!("increment_player_experience");
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
        for _ in level_up_rcv.read() {
            warn!("level_up");
            // Regen life
            let max_life = life.max_life();
            life.regenerate(max_life);
        }
    }
}
