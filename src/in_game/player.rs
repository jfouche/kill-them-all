use crate::in_game::bullets::{spawn_bullet_at, BulletOptions};
use crate::in_game::collisions::{GROUP_ENEMY, GROUP_PLAYER};
use crate::prelude::*;
use std::ops::Mul;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_movement)
                .with_system(animate_sprite)
                .with_system(player_fires)
                .with_system(on_player_hit)
                .with_system(set_invulnerable)
                .with_system(animate_invulnerability)
                .with_system(player_invulnerability_finished.after(animate_invulnerability))
                .with_system(increment_player_experience)
                .with_system(level_up),
        );
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
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

fn spawn_player(commands: &mut Commands, texture_atlas_handle: Handle<TextureAtlas>) {
    commands
        .spawn(Player)
        .insert(Name::new("Player"))
        .insert(MovementSpeed::new(8.))
        .insert(Life::new(10))
        .insert(Money(0))
        .insert(Experience::default())
        // Sprite
        .insert(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0., 0., 10.),
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
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 7, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    spawn_player(&mut commands, texture_atlas_handle);
    commands.insert_resource(PlayerFireConfig {
        timer: Timer::from_seconds(1., TimerMode::Repeating),
    });
}

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
    mut config: ResMut<PlayerFireConfig>,
    q_player: Query<&Transform, With<Player>>,
    q_monsters: Query<&Transform, With<Monster>>,
) {
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
    mut q_player: Query<(&mut Life, &mut CollisionGroups), With<Player>>,
    mut send_invulnerability: EventWriter<InvulnerabilityEvent>,
    mut send_death: EventWriter<PlayerDeathEvent>,
) {
    if let Ok((mut life, mut collision_groups)) = q_player.get_single_mut() {
        for event in player_hit_events.iter() {
            warn!("on_player_hit");
            life.hit(1);
            if life.is_dead() {
                commands.entity(event.entity).despawn();
                send_death.send(PlayerDeathEvent);
                // break to ensure we don't try to despawn player if already dead
                break;
            } else {
                send_invulnerability.send(InvulnerabilityEvent::Start(event.entity));
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

///
///
///
fn set_invulnerable(
    mut commands: Commands,
    mut events: EventReader<InvulnerabilityEvent>,
    mut q_player: Query<&mut CollisionGroups, With<Player>>,
) {
    if let Ok(mut collision_groups) = q_player.get_single_mut() {
        for event in events.iter() {
            if let InvulnerabilityEvent::Start(entity) = event {
                warn!("set_invulnerable");
                commands
                    .entity(*entity)
                    .insert(Invulnerable::new(0.5, GROUP_ENEMY))
                    .insert(InvulnerabilityAnimationTimer::default());

                // To allow player to not collide with enemies
                collision_groups.filters &= !GROUP_ENEMY;
            }
        }
    }
}

///
///
///
fn animate_invulnerability(
    time: Res<Time>,
    mut q_player: Query<
        (&mut Visibility, &mut InvulnerabilityAnimationTimer),
        (With<Player>, With<Invulnerable>),
    >,
) {
    if let Ok((mut visibility, mut timer)) = q_player.get_single_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            warn!("animate_invulnerability");
            visibility.is_visible = !visibility.is_visible;
        }
    }
}

///
///
///
fn player_invulnerability_finished(
    mut commands: Commands,
    mut events: EventReader<InvulnerabilityEvent>,
    mut q_player: Query<(Entity, &mut Visibility), With<Player>>,
) {
    if let Ok((player_entity, mut visibility)) = q_player.get_single_mut() {
        for event in events.iter() {
            if let InvulnerabilityEvent::Stop(entity) = event {
                if player_entity == *entity {
                    warn!("player_invulnerability_finished");
                    commands
                        .entity(player_entity)
                        .remove::<InvulnerabilityAnimationTimer>();
                    visibility.is_visible = true;
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
    mut q_player: Query<(&mut Life, &mut MovementSpeed), With<Player>>,
    mut level_up_rcv: EventReader<LevelUpEvent>,
) {
    if let Ok((mut life, mut movement_speed)) = q_player.get_single_mut() {
        for _ in level_up_rcv.iter() {
            warn!("level_up");
            life.increases(10.);
            movement_speed.increases(10.);
            // Regen life
            let max_life = life.max_life();
            life.regenerate(max_life);
        }
    }
}
