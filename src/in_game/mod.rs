mod bonus_plugin;
mod collisions_plugin;
mod hud;
mod level_up_menu;
mod monster_plugin;
mod pause_menu;
mod player_died_menu;
mod player_plugin;
mod round_plugin;
mod world_map_plugin;

use crate::components::{LifeTime, PlayerDeathEvent};
use crate::cursor::*;
use crate::schedule::*;
use crate::utils::invulnerable::Invulnerable;
use crate::utils::Blink;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct InGamePluginsGroup;

impl PluginGroup for InGamePluginsGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(hud::TopMenuPlugin)
            .add(bonus_plugin::BonusPlugin)
            .add(collisions_plugin::CollisionsPlugin)
            .add(monster_plugin::MonsterPlugin)
            .add(player_plugin::PlayerPlugin)
            .add(round_plugin::RoundPlugin)
            .add(world_map_plugin::WorldMapPlugin)
            .add(pause_menu::PausePlugin)
            .add(level_up_menu::LevelUpMenuPlugin)
            .add(player_died_menu::PlayerDiedPlugin)
            .add(in_game_schedule_plugin)
    }
}

fn in_game_schedule_plugin(app: &mut App) {
    app.add_systems(Startup, stop_physics)
        .add_systems(OnEnter(GameState::InGame), (run_game, grab_cursor))
        .add_systems(OnExit(GameState::InGame), (ungrab_cursor, reset_physics))
        .add_systems(OnEnter(InGameState::Running), (grab_cursor, start_physics))
        .add_systems(OnExit(InGameState::Running), (ungrab_cursor, stop_physics))
        .add_systems(OnEnter(InGameState::Pause), pause)
        .add_systems(OnExit(InGameState::Pause), unpause)
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

fn pause(mut blinks: Query<&mut Blink>, mut invulnerables: Query<&mut Invulnerable>) {
    for mut blink in &mut blinks {
        blink.pause(true);
    }
    for mut invulnerable in &mut invulnerables {
        invulnerable.pause(true);
    }
}

fn unpause(mut blinks: Query<&mut Blink>, mut invulnerables: Query<&mut Invulnerable>) {
    for mut blink in &mut blinks {
        blink.pause(false);
    }
    for mut invulnerable in &mut invulnerables {
        invulnerable.pause(false);
    }
}

fn start_physics(mut physics: ResMut<RapierConfiguration>) {
    physics.physics_pipeline_active = true;
    physics.query_pipeline_active = true;
}

fn stop_physics(mut physics: ResMut<RapierConfiguration>) {
    physics.physics_pipeline_active = false;
    physics.query_pipeline_active = false;
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
