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

#[derive(Component)]
struct BackToMenu;

#[derive(Component)]
struct EquipmentButton;

#[derive(Resource, Default)]
struct RoundEndMenuNav(Vec<Entity>);

impl std::ops::Deref for RoundEndMenuNav {
    type Target = [Entity];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn spawn_round_end_menu(mut commands: Commands) {
    let mut round_end_nav = RoundEndMenuNav::default();

    let id = commands.spawn_button("HELMET", (EquipmentButton, SelectedOption));
    round_end_nav.0.push(id);

    let id = commands.spawn_button("Back to game", BackToMenu);
    round_end_nav.0.push(id);

    commands
        .spawn_popup("End of round", (RoundEndMenu, Name::new("RoundEndMenu")))
        .push_children(&round_end_nav);

    commands.insert_resource(round_end_nav);
}

/// Handle the selection of an [Equipment], to add to the [Player]
fn select_equipment(
    mut players: Query<&mut Helmet, With<Player>>,
    mut state: ResMut<NextState<InGameState>>,
    interactions: Query<(&EquipmentButton, &Interaction)>,
) {
    let Ok(mut helmet) = players.get_single_mut() else {
        return;
    };
    for (_btn, interaction) in &interactions {
        if *interaction == Interaction::Pressed {
            *helmet = Helmet::NormalHelmet(NormalHelmet { armor: 1. });
            state.set(InGameState::Running);
        }
    }
}
