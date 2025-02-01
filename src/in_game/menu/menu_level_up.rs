use super::panel_equipments::EquipmentsPanel;
use crate::{
    components::{
        despawn_all,
        player::{LevelUpEvent, Player},
        upgrade::UpgradeProvider,
    },
    in_game::back_to_game,
    schedule::{GameRunningSet, InGameState},
    ui::{
        button::{button_keyboard_nav, SelectedOption, TextButton},
        popup::Popup,
        HSizer, VSizer,
    },
};
use bevy::prelude::*;

#[derive(Component)]
#[require(
    Popup(|| Popup::default().with_title("Level up!")),
    Name(|| Name::new("LevelUpMenu"))
)]
struct LevelUpMenu;

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
    let mut rng = rand::rng();

    for _ in 0..3 {
        if let Some(upgrade) = upgrade_provider.gen(&mut rng) {
            let upgrade_view = upgrade.generate(&mut commands, &mut rng);
            let btn_entity = commands
                .spawn((
                    TextButton::big(upgrade_view.label),
                    UpgradeEntity(upgrade_view.entity),
                ))
                .id();
            level_up_nav.0.push(btn_entity);
            upgrade_list.push(upgrade_view.entity);
        }
    }

    // Select the first upgrade
    if let Some(entity) = &level_up_nav.first() {
        commands.entity(**entity).insert(SelectedOption);
    }

    commands.spawn(LevelUpMenu).with_children(|menu| {
        menu.spawn(HSizer).with_children(|sizer| {
            sizer.spawn(VSizer).add_children(&level_up_nav);
            sizer.spawn(EquipmentsPanel);
        });
    });

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
/// Upgrade the player, returning back to game
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
                    // move upgrade to player
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
