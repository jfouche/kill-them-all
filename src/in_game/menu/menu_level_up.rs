use super::panel_equipments::EquipmentsPanel;
use crate::{
    components::{
        despawn_all,
        player::{LevelUpEvent, Player},
        upgrade::UpgradeProvider,
    },
    in_game::back_to_game,
    schedule::InGameState,
    ui::{
        button::TextButton,
        popup::{Popup, PopupTitle},
        HSizer, VSizer,
    },
};
use bevy::prelude::*;

#[derive(Component)]
struct LevelUpMenu;

#[derive(Resource, Default, Deref, DerefMut)]
struct UpgradeList(Vec<Entity>);

#[derive(Component, Deref)]
struct UpgradeEntity(Entity);

pub struct LevelUpMenuPlugin;

impl Plugin for LevelUpMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::LevelUp), spawn_level_up_menu)
            .add_systems(
                OnExit(InGameState::LevelUp),
                (despawn_all::<LevelUpMenu>, despawn_remaining_upgrades),
            )
            .add_systems(
                Update,
                (back_to_game, upgrade_skill).run_if(in_state(InGameState::LevelUp)),
            )
            .add_observer(enter_level_up_state);
    }
}

fn enter_level_up_state(
    _trigger: Trigger<LevelUpEvent>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    next_state.set(InGameState::LevelUp);
}

fn spawn_level_up_menu(mut commands: Commands) {
    let mut upgrade_list = UpgradeList::default();
    let mut upgrade_provider = UpgradeProvider::new();

    let mut upgrade_entities = Vec::new();
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
            upgrade_entities.push(btn_entity);
            upgrade_list.push(upgrade_view.entity);
        }
    }

    commands
        .spawn((
            LevelUpMenu,
            Name::new("LevelUpMenu"),
            Popup,
            children![PopupTitle::bundle("Level up!")],
        ))
        .with_children(|menu| {
            menu.spawn(HSizer).with_children(|sizer| {
                sizer.spawn(VSizer).add_children(&upgrade_entities);
                sizer.spawn(EquipmentsPanel);
            });
        });

    commands.insert_resource(upgrade_list);
}

/// Despawn all remaining upgrades
fn despawn_remaining_upgrades(mut commands: Commands, upgrade_list: Res<UpgradeList>) {
    for &entity in upgrade_list.iter() {
        commands.entity(entity).despawn();
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
    if let Ok(player) = players.single() {
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
