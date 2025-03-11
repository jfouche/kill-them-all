pub mod affix_updates_plugin;
pub mod animation_plugin;
pub mod character_plugin;
pub mod collisions_plugin;
pub mod hud;
pub mod item_plugin;
pub mod life_bar_plugin;
pub mod menu;
pub mod monster_plugin;
pub mod player_plugin;
pub mod skills;
pub mod world_map_plugin;

pub use plugin::{back_to_game, pause, unpause, InGamePluginsGroup};

mod plugin {
    use super::*;
    use crate::components::player::PlayerDeathEvent;
    use crate::components::LifeTime;
    use crate::schedule::{GameRunningSet, GameState, InGameState};
    use crate::utils::blink::Blink;
    use crate::utils::despawn_after::DespawnAfter;
    use crate::utils::invulnerable::Invulnerable;
    use bevy::app::PluginGroupBuilder;
    use bevy::prelude::*;
    use bevy_rapier2d::prelude::*;

    pub struct InGamePluginsGroup;

    impl PluginGroup for InGamePluginsGroup {
        fn build(self) -> PluginGroupBuilder {
            PluginGroupBuilder::start::<Self>()
                .add(hud::HudPlugin)
                .add(item_plugin::ItemPlugin)
                .add(collisions_plugin::CollisionsPlugin)
                .add(character_plugin::CharacterPlugin)
                .add(affix_updates_plugin::AffixUpdatesPlugin)
                .add(monster_plugin::MonsterPlugin)
                .add(player_plugin::PlayerPlugin)
                .add(world_map_plugin::WorldMapPlugin)
                .add(life_bar_plugin::LifeBarPlugin)
                .add(animation_plugin::AnimationPlugin)
                .add(skills::SkillsPlugin)
                .add(menu::InGameMenuPlugin)
                .add(in_game_schedule_plugin)
        }
    }

    fn in_game_schedule_plugin(app: &mut App) {
        app.add_systems(Startup, stop_physics)
            .add_systems(OnEnter(GameState::InGame), (run_game, init_physics))
            .add_systems(OnExit(GameState::InGame), reset_physics)
            .add_systems(OnEnter(InGameState::Running), start_physics)
            .add_systems(OnExit(InGameState::Running), stop_physics)
            .add_systems(Update, switch_to_pause.in_set(GameRunningSet::UserInput))
            .add_systems(
                Update,
                despawn_if_too_old.in_set(GameRunningSet::DespawnEntities),
            )
            .add_observer(change_state_on_player_death);
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

    pub fn pause(
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

    pub fn unpause(
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

    fn change_state_on_player_death(
        _trigger: Trigger<PlayerDeathEvent>,
        mut in_game_state: ResMut<NextState<InGameState>>,
    ) {
        in_game_state.set(InGameState::PlayerDied);
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
}
