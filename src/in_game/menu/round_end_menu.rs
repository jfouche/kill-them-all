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

trait EquipmentLabel {
    fn label(&self) -> String;
}

impl EquipmentLabel for Equipment {
    fn label(&self) -> String {
        match self {
            Equipment::Helmet(helmet) => helmet.to_string(),
            Equipment::BodyArmour(body_armour) => body_armour.to_string(),
            Equipment::Boots(boots) => boots.to_string(),
        }
    }
}

fn spawn_round_end_menu(mut commands: Commands, assets: Res<EquipmentAssets>) {
    let mut round_end_nav = RoundEndMenuNav::default();
    let mut equipment_provider = EquipmentProvider::new();

    for _ in 0..3 {
        if let Some(equipment) = equipment_provider.gen() {
            let (texture, atlas) = assets.image(&equipment);
            let img = ButtonImage::ImageAtlas(texture, atlas);
            let entity = commands.spawn_img_text_button(img, equipment.label(), equipment);
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
}

/// Handle the selection of an [Equipment], to add to the [Player]
fn select_equipment(
    mut players: Query<(&mut Helmet, &mut BodyArmour, &mut Boots), With<Player>>,
    mut state: ResMut<NextState<InGameState>>,
    interactions: Query<(&Equipment, &Interaction), With<Button>>,
) {
    let Ok((mut helmet, mut body_armour, mut boots)) = players.get_single_mut() else {
        return;
    };
    for (equipment, interaction) in &interactions {
        if *interaction == Interaction::Pressed {
            match equipment {
                Equipment::Helmet(new_helmet) => *helmet = new_helmet.clone(),
                Equipment::BodyArmour(new_body_armour) => *body_armour = new_body_armour.clone(),
                Equipment::Boots(new_boots) => *boots = new_boots.clone(),
            }
            state.set(InGameState::Running);
        }
    }
}
