use crate::components::*;
use crate::in_game::back_to_game;
use crate::schedule::*;
use crate::ui::*;
use bevy::prelude::*;

pub struct RoundEndMenuPlugin;

impl Plugin for RoundEndMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::RoundEnd), spawn_round_end_menu)
            .add_systems(OnExit(InGameState::RoundEnd), despawn_all::<RoundEndMenu>)
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

// trait EquipmentLabel {
//     fn label(&self) -> String;
// }

// impl EquipmentLabel for Equipment {
//     fn label(&self) -> String {
//         match self {
//             Equipment::Helmet(helmet) => helmet.to_string(),
//             Equipment::BodyArmour(body_armour) => body_armour.to_string(),
//             Equipment::Boots(boots) => boots.to_string(),
//         }
//     }
// }

fn spawn_round_end_menu(mut commands: Commands, assets: Res<EquipmentAssets>) {
    let mut equipment_list = EquipmentList::default();

    let mut round_end_nav = RoundEndMenuNav::default();
    let mut equipment_provider = EquipmentProvider::new();
    let mut rng = rand::thread_rng();

    for _ in 0..3 {
        if let Some(equipment) = equipment_provider.gen(&mut rng) {
            let equipment_entity = equipment.spawn(&mut commands, &mut rng);
            equipment_list.push(equipment_entity.entity);
            let texture = assets.texture();
            let atlas = assets.atlas(equipment_entity.tile_index);
            let img = ButtonImage::ImageAtlas(texture, atlas);
            let entity = commands.spawn_img_text_button(
                img,
                equipment_entity.label,
                EquipmentEntity(equipment_entity.entity),
            );
            round_end_nav.0.push(entity);
        }
    }

    // Select the first upgrade
    if let Some(entity) = &round_end_nav.first() {
        commands.entity(**entity).insert(SelectedOption);
    }

    commands
        .spawn_popup("End of round", (RoundEndMenu, Name::new("RoundEndMenu")))
        .push_children(&round_end_nav);

    commands.insert_resource(round_end_nav);
    commands.insert_resource(equipment_list);
}

/// Handle the selection of an equipment, to add to the [Player]
fn select_equipment(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    interactions: Query<(&EquipmentEntity, &Interaction), With<Button>>,
    equipment_list: ResMut<EquipmentList>,
    mut state: ResMut<NextState<InGameState>>,
) {
    let Ok(player_entity) = players.get_single() else {
        return;
    };
    for (equipment_entity, interaction) in &interactions {
        if *interaction == Interaction::Pressed {
            for &e in equipment_list.iter() {
                if e == **equipment_entity {
                    // move equipment to player
                    commands.entity(player_entity).add_child(e);
                } else {
                    // despawn unselected equipment
                    commands.entity(e).despawn_recursive();
                }
            }

            commands.remove_resource::<EquipmentList>();
            state.set(InGameState::Running);
        }
    }
}
