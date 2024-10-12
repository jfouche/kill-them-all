use crate::components::*;
use crate::in_game::back_to_game;
use crate::schedule::*;
use crate::ui::*;
use bevy::prelude::*;

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
            .add_systems(OnExit(InGameState::LevelUp), despawn_all::<LevelUpMenu>)
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

#[derive(Component)]
struct LevelUpMenu;

///
/// Trait to print a player skill label on a button
///
trait ButtonLabel {
    /// label to display
    fn label(&self) -> String;
}

impl ButtonLabel for Upgrade {
    fn label(&self) -> String {
        match self {
            Upgrade::IncreaseMaxLife(val) => format!("+{:.0}% max life", val),
            Upgrade::IncreasemovementSpeed(val) => format!("+{:.0}% movement speed", val),
            Upgrade::IncreaseLifeRegen(val) => format!("+{:.0}% life regen", val),
            Upgrade::IncreaseAttackSpeed(val) => format!("+{:.0}% attack speed", val),
            Upgrade::PierceChance(val) => format!("+{:.0}% chance to pierce", val),
        }
    }
}

#[derive(Resource, Default)]
struct LevelUpMenuNav(Vec<Entity>);

impl std::ops::Deref for LevelUpMenuNav {
    type Target = [Entity];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn spawn_level_up_menu(mut commands: Commands) {
    let mut level_up_nav = LevelUpMenuNav::default();
    let mut upgrade_provider = UpgradeProvider::new();

    for _ in 0..3 {
        if let Some(upgrade) = upgrade_provider.gen() {
            let label = upgrade.label();
            let entity = commands.spawn_text_button(label, upgrade);
            level_up_nav.0.push(entity);
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
    mut q_btn: Query<(&Interaction, &Upgrade), Changed<Interaction>>,
    mut q_player: Query<&mut Upgrades, With<Player>>,
    mut state: ResMut<NextState<InGameState>>,
) {
    if let Ok(mut upgrades) = q_player.get_single_mut() {
        for (interaction, ugrade) in &mut q_btn {
            if *interaction == Interaction::Pressed {
                upgrades.push(*ugrade);
                state.set(InGameState::Running);
            }
        }
    }
}
