use super::panel_inventory::inventory_panel;
use crate::components::*;
use crate::in_game::back_to_game;
use crate::schedule::*;
use crate::ui::*;
use bevy::prelude::*;

#[derive(Component)]
struct LevelUpMenu;

fn level_up_menu_bundle() -> impl Bundle {
    (LevelUpMenu, Name::new("LevelUpMenu"))
}

#[derive(Resource, Default)]
struct LevelUpMenuNav(Vec<Entity>);

impl std::ops::Deref for LevelUpMenuNav {
    type Target = [Entity];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct UpgradeList(Vec<Entity>);

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
            .add_systems(OnEnter(InGameState::LevelUp), spawn_level_up_menu)
            .add_systems(
                OnExit(InGameState::LevelUp),
                (despawn_all::<LevelUpMenu>, despawn_remaining_upgrades),
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

fn spawn_level_up_menu(mut commands: Commands) {
    let mut upgrade_list = UpgradeList::default();

    let mut level_up_nav = LevelUpMenuNav::default();
    let mut upgrade_provider = UpgradeProvider::new();
    let mut rng = rand::thread_rng();

    for _ in 0..3 {
        if let Some(upgrade) = upgrade_provider.gen(&mut rng) {
            let upgrade_view = upgrade.generate(&mut commands, &mut rng);
            let btn_entity =
                commands.spawn_text_button(upgrade_view.label, UpgradeEntity(upgrade_view.entity));
            level_up_nav.0.push(btn_entity);
            upgrade_list.push(upgrade_view.entity);
        }
    }

    // Select the first upgrade
    if let Some(entity) = &level_up_nav.first() {
        commands.entity(**entity).insert(SelectedOption);
    }

    let level_up_panel = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .push_children(&level_up_nav)
        .id();

    let inventory_panel = commands.spawn(inventory_panel()).id();

    commands
        .spawn_popup("Level up!", level_up_menu_bundle())
        .push_children(&[level_up_panel, inventory_panel]);

    commands.insert_resource(level_up_nav);
    commands.insert_resource(upgrade_list);
}

/// Despawn all remaining upgrades
fn despawn_remaining_upgrades(mut commands: Commands, upgrade_list: Res<UpgradeList>) {
    for &entity in upgrade_list.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<UpgradeList>();
}

///
/// Upgrade a skill of the player, returning back to game
///
fn upgrade_skill(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    interactions: Query<(&Interaction, &UpgradeEntity), Changed<Interaction>>,
    mut upgrade_list: ResMut<UpgradeList>,
    mut state: ResMut<NextState<InGameState>>,
) {
    if let Ok(player) = players.get_single() {
        for (interaction, upgrade_entity) in &interactions {
            if *interaction == Interaction::Pressed {
                if let Some(i) = upgrade_list.iter().position(|&e| e == **upgrade_entity) {
                    // move equipment to player
                    commands.entity(player).add_child(**upgrade_entity);
                    // Remove it from the list of entity to despawn
                    upgrade_list.swap_remove(i);
                    // leave the menu and go back to game
                    state.set(InGameState::Running);
                }
            }
        }
    }
}
