use super::panel_equipments::EquipmentsPanel;
use crate::{
    components::{
        despawn_all,
        player::{LevelUpEvent, Player},
        upgrade::UpgradeList,
    },
    in_game::back_to_game,
    schedule::InGameState,
    theme::widget,
    ui::{
        popup::{Popup, PopupTitle},
        HSizer, VSizer,
    },
};
use bevy::{ecs::spawn::SpawnWith, prelude::*};

pub struct LevelUpMenuPlugin;

impl Plugin for LevelUpMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::LevelUp), spawn_level_up_menu)
            .add_systems(
                OnExit(InGameState::LevelUp),
                (despawn_all::<LevelUpMenu>, despawn_remaining_upgrades),
            )
            .add_systems(Update, back_to_game.run_if(in_state(InGameState::LevelUp)))
            .add_observer(enter_level_up_state);
    }
}

#[derive(Component)]
struct LevelUpMenu;

fn level_up_menu(upgrade_list: &UpgradeList) -> impl Bundle {
    let upgrade_labels = upgrade_list
        .iter()
        .map(|u| u.label.clone())
        .collect::<Vec<_>>();
    (
        LevelUpMenu,
        Name::new("LevelUpMenu"),
        Popup,
        children![
            PopupTitle::bundle("Level up!"),
            (
                HSizer,
                children![
                    (
                        VSizer,
                        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                            for (i, label) in upgrade_labels.iter().enumerate() {
                                parent.spawn(button_upgrade(label.clone(), i));
                            }
                        })),
                    ),
                    EquipmentsPanel
                ]
            ),
        ],
    )
}

fn button_upgrade(label: impl Into<String>, index: usize) -> impl Bundle {
    let observer = move |_t: Trigger<Pointer<Click>>,
                         mut commands: Commands,
                         players: Query<Entity, With<Player>>,
                         mut upgrade_list: ResMut<UpgradeList>,
                         mut state: ResMut<NextState<InGameState>>| {
        let player = players.single()?;
        upgrade_list.upgrade(commands.entity(player), index);
        state.set(InGameState::Running);
        Ok(())
    };
    widget::button(label, observer)
}

fn enter_level_up_state(
    _trigger: Trigger<LevelUpEvent>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    next_state.set(InGameState::LevelUp);
}

fn spawn_level_up_menu(mut commands: Commands) {
    let mut rng = rand::rng();
    let upgrade_list = UpgradeList::new(&mut commands, &mut rng);
    commands.spawn(level_up_menu(&upgrade_list));
    commands.insert_resource(upgrade_list);
}

/// Despawn all remaining upgrades
fn despawn_remaining_upgrades(mut commands: Commands, upgrade_list: Res<UpgradeList>) {
    for upgrade_view in upgrade_list.iter() {
        commands.entity(upgrade_view.entity).despawn();
    }
    commands.remove_resource::<UpgradeList>();
}
