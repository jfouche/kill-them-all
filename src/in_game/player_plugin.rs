use crate::camera::MainCamera;
use crate::components::*;
use crate::schedule::*;
use crate::ui::mouse_over_ui::mouse_not_over_ui;
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
            .init_resource::<AllowMouseHandling>()
            .add_event::<PlayerDeathEvent>()
            .add_event::<InventoryChanged>()
            .add_event::<PlayerEquipmentChanged>()
            .register_type::<Experience>()
            .register_type::<Inventory>()
            .register_type::<InventoryPos>()
            .add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(
                OnExit(GameState::InGame),
                (despawn_all::<Player>, despawn_all::<Inventory>),
            )
            .add_systems(OnEnter(GameState::InGame), unpause)
            .add_systems(OnExit(GameState::InGame), pause)
            .add_systems(OnEnter(InGameState::Running), disable_mouse_handling)
            .add_systems(
                Update,
                (
                    (
                        wait_for_mouse_up,
                        set_target_position.run_if(mouse_not_over_ui),
                    )
                        .chain(),
                    animate_player_sprite,
                    player_invulnerability_finished,
                    increment_player_experience,
                    level_up,
                )
                    .in_set(GameRunningSet::EntityUpdate),
            );
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct AllowMouseHandling(bool);

fn disable_mouse_handling(mut allow_mouse: ResMut<AllowMouseHandling>) {
    **allow_mouse = false;
}

fn wait_for_mouse_up(
    mouse_inputs: Res<ButtonInput<MouseButton>>,
    mut allow_mouse: ResMut<AllowMouseHandling>,
) {
    if !**allow_mouse && !mouse_inputs.pressed(MouseButton::Left) {
        **allow_mouse = true;
    }
}

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands.spawn(Inventory::default());

    commands
        .spawn((Player, Player::sprite(&assets)))
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

///
/// Manage the mouse to move the player
///
fn set_target_position(
    mut commands: Commands,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    player: Single<&mut NextPosition, With<Player>>,
    mouse_inputs: Res<ButtonInput<MouseButton>>,
    allow_mouse: Res<AllowMouseHandling>,
    assets: Res<NextPositionIndicatorAssets>,
) {
    if !**allow_mouse || !mouse_inputs.pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = *camera;
    let Some(point) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    else {
        return;
    };

    let mut next_pos = player.into_inner();
    next_pos.goto(point);

    if mouse_inputs.just_pressed(MouseButton::Left) {
        commands.spawn((
            NextPositionIndicator,
            Mesh2d(assets.mesh.clone()),
            MeshMaterial2d(assets.color.clone()),
            Transform::from_translation((point, 10.).into()),
        ));
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
            Invulnerable::new(Duration::from_secs_f32(2.0), GROUP_ENEMY),
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
