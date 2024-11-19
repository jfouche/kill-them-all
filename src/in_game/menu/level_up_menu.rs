use crate::components::*;
use crate::in_game::back_to_game;
use crate::schedule::*;
use crate::ui::*;
use bevy::prelude::*;

#[derive(Component)]
struct LevelUpMenu;

#[derive(Resource, Default)]
struct LevelUpMenuNav(Vec<Entity>);

impl std::ops::Deref for LevelUpMenuNav {
    type Target = [Entity];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Component)]
struct UpgradesContainer;

#[derive(Component, Deref)]
struct UpgradeEntity(Entity);

pub struct LevelUpMenuPlugin;

impl Plugin for LevelUpMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelUpMenuNav>()
            .add_event::<LevelUpEvent>()
            .add_systems(
                Update,
                enter_level_up_state.in_set(GameRunningSet::EntityUpdate),
            )
            .add_systems(
                OnEnter(InGameState::LevelUp),
                (spawn_upgrades_container, spawn_level_up_menu).chain(),
            )
            .add_systems(
                OnExit(InGameState::LevelUp),
                (despawn_all::<LevelUpMenu>, despawn_all::<UpgradesContainer>),
            )
            .add_systems(
                Update,
                (
                    button_keyboard_nav::<LevelUpMenuNav>,
                    (back_to_game, upgrade_skill),
                )
                    .chain()
                    .run_if(in_state(InGameState::LevelUp)),
            );
    }
}

fn enter_level_up_state(
    mut level_up_rcv: EventReader<LevelUpEvent>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if level_up_rcv.read().next().is_some() {
        next_state.set(InGameState::LevelUp);
    }
}

fn spawn_upgrades_container(mut commands: Commands) {
    commands.spawn((UpgradesContainer, Name::new("UpgradesContainer")));
}

fn spawn_level_up_menu(mut commands: Commands, upgrades: Query<Entity, With<UpgradesContainer>>) {
    let upgrades_container = upgrades
        .get_single()
        .expect("UpgradesContainer is not spawned");

    let mut level_up_nav = LevelUpMenuNav::default();
    let mut upgrade_provider = UpgradeProvider::new();
    let mut rng = rand::thread_rng();
    for _ in 0..3 {
        if let Some(upgrade) = upgrade_provider.gen(&mut rng) {
            let upgrade_view = upgrade.generate(&mut commands, &mut rng);
            let btn_entity =
                commands.spawn_text_button(upgrade_view.label, UpgradeEntity(upgrade_view.entity));
            level_up_nav.0.push(btn_entity);
            commands
                .entity(upgrades_container)
                .add_child(upgrade_view.entity);
        }
    }

    // Select the first upgrade
    if let Some(entity) = &level_up_nav.first() {
        commands.entity(**entity).insert(SelectedOption);
    }

    commands
        .spawn_popup("Level up!", (LevelUpMenu, Name::new("LevelUpMenu")))
        .push_children(&level_up_nav);

    commands.insert_resource(level_up_nav);
}

///
/// Upgrade a skill of the player, returning back to game
///
fn upgrade_skill(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    interactions: Query<(Entity, &Interaction, &UpgradeEntity), Changed<Interaction>>,
    mut state: ResMut<NextState<InGameState>>,
) {
    if let Ok(player) = players.get_single() {
        for (btn, interaction, upgrade_entity) in &interactions {
            if *interaction == Interaction::Pressed {
                // move upgrades to player
                commands.entity(btn).remove_children(&[**upgrade_entity]);
                commands.entity(player).add_child(**upgrade_entity);
                state.set(InGameState::Running);
            }
        }
    }
}
