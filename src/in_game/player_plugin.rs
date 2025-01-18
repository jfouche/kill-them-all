use crate::camera::MainCamera;
use crate::components::*;
use crate::schedule::*;
use crate::utils::blink::Blink;
use crate::utils::invulnerable::Invulnerable;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAssets>()
            .init_resource::<NextPositionIndicatorAssets>()
            .init_resource::<Score>()
            // .init_resource::<AllowMouseHandling>()
            .add_event::<PlayerDeathEvent>()
            .add_event::<InventoryChanged>()
            .add_event::<PlayerEquipmentChanged>()
            .register_type::<Experience>()
            .register_type::<Inventory>()
            .register_type::<InventoryPos>()
            .add_systems(
                OnExit(GameState::InGame),
                (despawn_all::<Player>, despawn_all::<Inventory>),
            )
            .add_systems(OnEnter(GameState::InGame), unpause)
            .add_systems(OnExit(GameState::InGame), pause)
            .add_systems(
                Update,
                (
                    animate_player_sprite,
                    player_invulnerability_finished,
                    increment_player_experience,
                    level_up,
                )
                    .in_set(GameRunningSet::EntityUpdate),
            )
            .add_observer(spawn_player)
            .add_observer(manage_player_movement_with_mouse);
    }
}

fn world_position<E>(
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    pointer: &Pointer<E>,
) -> Option<Vec2>
where
    E: std::fmt::Debug + Clone + Reflect,
{
    cameras.get_single().ok().and_then(|(camera, transform)| {
        camera
            .viewport_to_world_2d(transform, pointer.pointer_location.position)
            .ok()
    })
}

fn manage_player_movement_with_mouse(trigger: Trigger<OnAdd, WorldMap>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(
            |trigger: Trigger<Pointer<Down>>,
             mut commands: Commands,
             mut player: Single<&mut NextPosition, With<Player>>,
             cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
             assets: Res<NextPositionIndicatorAssets>| {
                if let Some(world_pos) = world_position(cameras, trigger.event()) {
                    player.goto(world_pos);
                    commands.spawn((
                        NextPositionIndicator,
                        Mesh2d(assets.mesh.clone()),
                        MeshMaterial2d(assets.color.clone()),
                        Transform::from_translation(world_pos.extend(10.)),
                    ));
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Drag>>,
             mut player: Single<&mut NextPosition, With<Player>>,
             cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>| {
                if let Some(world_pos) = world_position(cameras, trigger.event()) {
                    player.goto(world_pos);
                }
            },
        );
}

fn spawn_player(
    trigger: Trigger<SpawnPlayerEvent>,
    mut commands: Commands,
    assets: Res<PlayerAssets>,
) {
    commands.spawn(Inventory::default());

    commands
        .spawn((
            Player,
            Transform::from_translation(trigger.event().translation.extend(LAYER_PLAYER)),
            Player::sprite(&assets),
        ))
        .with_children(|player| {
            player.spawn(DeathAura);
            player.spawn(IncreaseAreaOfEffect(50.));
        })
        .observe(set_invulnerable_on_hit)
        .observe(player_dying);
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

fn set_invulnerable_on_hit(
    trigger: Trigger<LooseLifeEvent>,
    mut commands: Commands,
    mut players: Query<&mut CollisionGroups, With<Player>>,
) {
    if let Ok(mut collision_groups) = players.get_mut(trigger.entity()) {
        // Set player invulnerable
        commands.entity(trigger.entity()).insert((
            Invulnerable::new(Duration::from_secs_f32(1.0), GROUP_ENEMY),
            Blink::new(Duration::from_secs_f32(0.15)),
        ));

        // To allow player to not collide with enemies
        collision_groups.filters &= !GROUP_ENEMY;
    }
}

fn player_dying(
    trigger: Trigger<CharacterDyingEvent>,
    mut commands: Commands,
    mut send_died: EventWriter<CharacterDiedEvent>,
) {
    info!("player_dying");
    commands.trigger(PlayerDeathEvent);
    send_died.send(CharacterDiedEvent(trigger.entity()));
}

///
/// Animate the player sprite
///
fn animate_player_sprite(
    time: Res<Time>,
    mut q_player: Query<(&Velocity, &mut AnimationTimer, &mut Sprite), With<Player>>,
) {
    if let Ok((&velocity, mut timer, mut sprite)) = q_player.get_single_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
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
            // info!("increment_player_experience");
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
