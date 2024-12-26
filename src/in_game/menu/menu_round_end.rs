use super::panel_inventory::InventoryPanel;
use crate::components::*;
use crate::in_game::back_to_game;
use crate::schedule::*;
use crate::ui::*;
use bevy::prelude::*;

pub struct RoundEndMenuPlugin;

impl Plugin for RoundEndMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::RoundEnd), spawn_round_end_menu)
            .add_systems(
                OnExit(InGameState::RoundEnd),
                (despawn_all::<RoundEndMenu>, despawn_remaining_equipments),
            )
            .add_systems(
                Update,
                (
                    button_keyboard_nav::<RoundEndMenuNav>,
                    (select_equipment, back_to_game),
                )
                    .chain()
                    .run_if(in_state(InGameState::RoundEnd)),
            );
    }
}

#[derive(Component)]
#[require(
    Popup(|| Popup::default().with_title("End of round")),
    Name(|| Name::new("RoundEndMenu"))
)]
struct RoundEndMenu;

#[derive(Resource, Default, DerefMut)]
struct RoundEndMenuNav(Vec<Entity>);

impl std::ops::Deref for RoundEndMenuNav {
    type Target = [Entity];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct EquipmentList(Vec<Entity>);

#[derive(Component, Deref)]
struct EquipmentEntity(Entity);

fn spawn_round_end_menu(mut commands: Commands, assets: Res<EquipmentAssets>) {
    let mut equipment_list = EquipmentList::default();

    let mut round_end_nav = RoundEndMenuNav::default();
    let mut equipment_provider = EquipmentProvider::new();
    let mut rng = rand::thread_rng();

    for _ in 0..3 {
        if let Some(equipment) = equipment_provider.gen(&mut rng) {
            let equipment_entity = equipment.spawn(&mut commands, &mut rng);
            equipment_list.push(equipment_entity.entity);
            let entity = commands
                .spawn((
                    MyButton::new(equipment_entity.info.text)
                        .with_image(assets.image_node(equipment_entity.info.tile_index)),
                    EquipmentEntity(equipment_entity.entity),
                ))
                .id();
            round_end_nav.0.push(entity);
        }
    }

    // Select the first upgrade
    if let Some(entity) = &round_end_nav.first() {
        commands.entity(**entity).insert(SelectedOption);
    }

    let level_up_panel = commands.spawn(VSizer).add_children(&round_end_nav).id();

    let inventory_panel = commands.spawn(InventoryPanel).id();

    commands.spawn(RoundEndMenu).with_children(|menu| {
        menu.spawn(HSizer)
            .add_children(&[level_up_panel, inventory_panel]);
    });

    commands.insert_resource(round_end_nav);
    commands.insert_resource(equipment_list);
}

/// Despawn all remaining equipments
fn despawn_remaining_equipments(mut commands: Commands, equipment_list: Res<EquipmentList>) {
    for &entity in equipment_list.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<EquipmentList>();
}

/// Handle the selection of an equipment, to add to the [Player]
fn select_equipment(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    interactions: Query<(&EquipmentEntity, &Interaction), With<Button>>,
    mut equipment_list: ResMut<EquipmentList>,
    mut state: ResMut<NextState<InGameState>>,
) {
    let Ok(player_entity) = players.get_single() else {
        return;
    };
    for (equipment_entity, interaction) in &interactions {
        if *interaction == Interaction::Pressed {
            if let Some(i) = equipment_list.iter().position(|&e| e == **equipment_entity) {
                // move equipment to player
                commands.entity(player_entity).add_child(**equipment_entity);
                // Remove it from the list of entity to despawn
                equipment_list.swap_remove(i);
                // leave the menu and go back to game
                state.set(InGameState::Running);
            }
        }
    }
}
