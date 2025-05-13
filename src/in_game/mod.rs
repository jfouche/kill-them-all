pub mod affix_updates_plugin;
pub mod animation_plugin;
pub mod character_plugin;
pub mod collisions_plugin;
pub mod dnd;
pub mod hud;
pub mod item_plugin;
pub mod life_bar_plugin;
pub mod menu;
pub mod monster_plugin;
pub mod orb_plugin;
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
    use avian2d::prelude::*;
    use bevy::app::PluginGroupBuilder;
    use bevy::prelude::*;

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
                .add(orb_plugin::OrbPlugin)
                .add(player_plugin::PlayerPlugin)
                .add(world_map_plugin::WorldMapPlugin)
                .add(life_bar_plugin::LifeBarPlugin)
                .add(animation_plugin::AnimationPlugin)
                .add(skills::SkillsPlugin)
                .add(dnd::DndPlugin)
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

    fn init_physics(mut gravity: ResMut<Gravity>) {
        info!("init_physics");
        *gravity = Gravity(Vec2::ZERO);
    }

    fn start_physics(mut time: ResMut<Time<Physics>>) {
        time.unpause();
    }

    fn stop_physics(mut time: ResMut<Time<Physics>>) {
        time.pause();
    }

    fn reset_physics(mut _commands: Commands) {
        // commands.insert_resource(Events::<CollisionEvent>::default());
        // commands.insert_resource(Events::<ContactForceEvent>::default());
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
                commands.entity(entity).despawn();
            }
        }
    }
}
