use crate::components::*;
use crate::in_game::back_to_game;
use crate::schedule::*;
use crate::ui::*;
use bevy::prelude::*;
use std::marker::PhantomData;

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
                    (
                        back_to_game,
                        upgrade_skill::<MaxLifeButton>,
                        upgrade_skill::<LifeRegenButton>,
                        upgrade_skill::<MovementSpeedButton>,
                        upgrade_skill::<AttackSpeedButton>,
                        upgrade_skill::<PierceChanceButton>,
                    ),
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
/// Trait to upgrade a player skill
///
trait UpgradeSkill {
    /// Component of the player skill
    type SkillComponent: Component;

    /// upgrade the skill
    fn upgrade(&self, component: &mut Self::SkillComponent);
}

///
/// Trait to print a player skill label on a button
///
trait UpgradeSkillButton {
    /// label to display
    fn label(&self) -> String;
}

/// Generic button to increase a skill
#[derive(Component)]
struct IncreaseButton<T> {
    increase: f32,
    _skill: PhantomData<T>,
}

impl<T> IncreaseButton<T> {
    fn new(increase: f32) -> Self {
        IncreaseButton {
            increase,
            _skill: PhantomData::<T>,
        }
    }
}

impl<T> UpgradeSkill for IncreaseButton<T>
where
    T: Component + Increase,
{
    type SkillComponent = T;
    fn upgrade(&self, component: &mut Self::SkillComponent) {
        component.increase(self.increase);
    }
}

type MaxLifeButton = IncreaseButton<Life>;

impl UpgradeSkillButton for MaxLifeButton {
    fn label(&self) -> String {
        format!("Increase max life : +{:.0}%", self.increase)
    }
}
type LifeRegenButton = IncreaseButton<LifeRegen>;

impl UpgradeSkillButton for LifeRegenButton {
    fn label(&self) -> String {
        format!("Increase life regen : +{:.0}%", self.increase)
    }
}

type AttackSpeedButton = IncreaseButton<AttackSpeed>;

impl UpgradeSkillButton for AttackSpeedButton {
    fn label(&self) -> String {
        format!("Increase attack speed : +{:.0}%", self.increase)
    }
}

type MovementSpeedButton = IncreaseButton<MovementSpeed>;

impl UpgradeSkillButton for MovementSpeedButton {
    fn label(&self) -> String {
        format!("Increase movement speed : +{:.0}%", self.increase)
    }
}

type PierceChanceButton = IncreaseButton<PierceChance>;

impl UpgradeSkillButton for PierceChanceButton {
    fn label(&self) -> String {
        format!("Increase pierce chance : +{:.0}%", self.increase)
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
            let entity = match upgrade {
                Upgrade::IncreaseAttackSpeed(increase) => {
                    spawn_upgrade_button(&mut commands, AttackSpeedButton::new(increase))
                }
                Upgrade::IncreaseMaxLife(increase) => {
                    spawn_upgrade_button(&mut commands, MaxLifeButton::new(increase))
                }
                Upgrade::IncreaseLifeRegen(increase) => {
                    spawn_upgrade_button(&mut commands, LifeRegenButton::new(increase))
                }
                Upgrade::IncreasemovementSpeed(increase) => {
                    spawn_upgrade_button(&mut commands, MovementSpeedButton::new(increase))
                }
                Upgrade::Pierce(increase) => {
                    spawn_upgrade_button(&mut commands, PierceChanceButton::new(increase))
                }
            };
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

fn spawn_upgrade_button<B>(parent: &mut Commands, button: B) -> Entity
where
    B: Component + UpgradeSkillButton,
{
    parent.spawn_button(button.label(), button)
}

///
/// Upgrade a skill of the player, returning back to game
///
fn upgrade_skill<T: UpgradeSkill + Component>(
    mut q_btn: Query<(&Interaction, &T), Changed<Interaction>>,
    mut q_player: Query<&mut T::SkillComponent, With<Player>>,
    mut state: ResMut<NextState<InGameState>>,
) {
    if let Ok(mut skill) = q_player.get_single_mut() {
        for (interaction, btn) in &mut q_btn {
            if *interaction == Interaction::Pressed {
                btn.upgrade(&mut skill);
                state.set(InGameState::Running);
            }
        }
    }
}
