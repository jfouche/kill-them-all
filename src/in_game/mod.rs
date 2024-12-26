mod animation_plugin;
mod bonus_plugin;
mod character_plugin;
mod collisions_plugin;
mod hud_plugin;
mod life_bar_plugin;
mod menu;
mod monster_plugin;
mod player_plugin;
mod round_plugin;
mod skill_plugin;
mod weapon_plugin;
mod world_map_plugin;

use crate::components::{LifeTime, PlayerDeathEvent, Round};
use crate::schedule::*;
use crate::utils::blink::Blink;
use crate::utils::cursor::*;
use crate::utils::despawn_after::DespawnAfter;
use crate::utils::invulnerable::Invulnerable;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct InGamePluginsGroup;

impl PluginGroup for InGamePluginsGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(hud_plugin::TopMenuPlugin)
            .add(bonus_plugin::BonusPlugin)
            .add(collisions_plugin::CollisionsPlugin)
            .add(character_plugin::CharacterPlugin)
            .add(monster_plugin::MonsterPlugin)
            .add(player_plugin::PlayerPlugin)
            .add(round_plugin::RoundPlugin)
            .add(world_map_plugin::WorldMapPlugin)
            .add(life_bar_plugin::LifeBarPlugin)
            .add(animation_plugin::AnimationPlugin)
            .add(weapon_plugin::WeaponPlugin)
            .add(skill_plugin::SkillsPlugin)
            .add_group(menu::InGameMenuPluginsGroup)
            .add(in_game_schedule_plugin)
    }
}

fn in_game_schedule_plugin(app: &mut App) {
    app.register_type::<Round>()
        .add_systems(Startup, stop_physics)
        .add_systems(
            OnEnter(GameState::InGame),
            (run_game, grab_cursor, init_physics),
        )
        .add_systems(OnExit(GameState::InGame), (ungrab_cursor, reset_physics))
        .add_systems(OnEnter(InGameState::Running), (grab_cursor, start_physics))
        .add_systems(OnExit(InGameState::Running), (ungrab_cursor, stop_physics))
        .add_systems(Update, switch_to_pause.in_set(GameRunningSet::UserInput))
        .add_systems(Update, on_player_death.in_set(GameRunningSet::EntityUpdate))
        .add_systems(
            Update,
            despawn_if_too_old.in_set(GameRunningSet::DespawnEntities),
        );
}

fn run_game(mut state: ResMut<NextState<InGameState>>) {
    state.set(InGameState::Running);
}

pub fn back_to_game(state: ResMut<NextState<InGameState>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        run_game(state);
    }
}

fn switch_to_pause(mut state: ResMut<NextState<InGameState>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        state.set(InGameState::Pause);
    }
}

fn pause(
    mut blinks: Query<&mut Blink>,
    mut invulnerables: Query<&mut Invulnerable>,
    mut despawnables: Query<&mut DespawnAfter>,
) {
    for mut blink in &mut blinks {
        blink.pause(true);
    }
    for mut invulnerable in &mut invulnerables {
        invulnerable.pause(true);
    }
    for mut despawnable in &mut despawnables {
        despawnable.pause(true);
    }
}

fn unpause(
    mut blinks: Query<&mut Blink>,
    mut invulnerables: Query<&mut Invulnerable>,
    mut despawnables: Query<&mut DespawnAfter>,
) {
    for mut blink in &mut blinks {
        blink.pause(false);
    }
    for mut invulnerable in &mut invulnerables {
        invulnerable.pause(false);
    }
    for mut despawnable in &mut despawnables {
        despawnable.pause(false);
    }
}

fn init_physics(mut conf: Query<&mut RapierConfiguration>) {
    if let Ok(mut conf) = conf.get_single_mut() {
        info!("init_physics");
        conf.gravity = Vect::ZERO;
    }
}

fn start_physics(mut physics: Query<&mut RapierConfiguration>) {
    if let Ok(mut physics) = physics.get_single_mut() {
        physics.physics_pipeline_active = true;
        physics.query_pipeline_active = true;
    }
}

fn stop_physics(mut physics: Query<&mut RapierConfiguration>) {
    if let Ok(mut physics) = physics.get_single_mut() {
        physics.physics_pipeline_active = false;
        physics.query_pipeline_active = false;
    }
}

fn reset_physics(mut commands: Commands) {
    commands.insert_resource(Events::<CollisionEvent>::default());
    commands.insert_resource(Events::<ContactForceEvent>::default());
}

fn on_player_death(
    mut player_death_events: EventReader<PlayerDeathEvent>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    for _ in player_death_events.read() {
        in_game_state.set(InGameState::PlayerDied);
    }
}

pub fn despawn_if_too_old(
    mut commands: Commands,
    mut query: Query<(Entity, &mut LifeTime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut query {
        if lifetime.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
