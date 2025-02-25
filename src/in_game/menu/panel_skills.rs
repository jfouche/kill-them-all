use crate::components::{
    item::ItemLocation,
    player::Player,
    skills::{
        death_aura::DeathAura, fireball::FireBallLauncher, mine::MineDropper,
        shuriken::ShurikenLauncher, SkillUI,
    },
};
use bevy::prelude::*;

#[derive(Component)]
#[require(
    Name(|| Name::new("SkillsPanel")),
    Node(|| Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(5.)),
        ..Default::default()
    })
)]
pub struct SkillsPanel;

pub struct SkillsPanelPlugin;

impl Plugin for SkillsPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(create_panel);
    }
}

fn create_panel(trigger: Trigger<OnAdd, SkillsPanel>, mut commands: Commands) {
    commands.entity(trigger.entity()).with_children(|panel| {
        panel.spawn(Text::new("A:"));
        panel.spawn(ItemLocation);
        panel.spawn(Text::new("Z:"));
        panel.spawn(ItemLocation);
        panel.spawn(Text::new("E:"));
        panel.spawn(ItemLocation);
        panel.spawn(Text::new("R:"));
        panel.spawn(ItemLocation);
    });
}

// fn show_skills(
//     trigger: Trigger<OnAdd, SkillsPanel>,
//     mut commands: Commands,
//     player: Single<Entity, With<Player>>,
//     fireballs: Query<&Parent, With<FireBallLauncher>>,
//     shurikens: Query<&Parent, With<ShurikenLauncher>>,
//     mines: Query<&Parent, With<MineDropper>>,
//     death_auras: Query<&Parent, With<DeathAura>>,
//     assets: Res<SkillAssets>,
// ) {
//     let player = *player;
//     let mut panel_commands = commands.entity(trigger.entity());
//     fireballs
//         .iter()
//         .filter(|&parent| **parent == player)
//         .for_each(|_| {
//             spawn_skill::<FireBallLauncher>(&mut panel_commands, &assets);
//         });
//     shurikens
//         .iter()
//         .filter(|&parent| **parent == player)
//         .for_each(|_| {
//             spawn_skill::<ShurikenLauncher>(&mut panel_commands, &assets);
//         });
//     mines
//         .iter()
//         .filter(|&parent| **parent == player)
//         .for_each(|_| {
//             spawn_skill::<MineDropper>(&mut panel_commands, &assets);
//         });
//     death_auras
//         .iter()
//         .filter(|&parent| **parent == player)
//         .for_each(|_| {
//             spawn_skill::<DeathAura>(&mut panel_commands, &assets);
//         });
// }

// fn spawn_skill<T>(panel: &mut EntityCommands, assets: &SkillAssets)
// where
//     T: Component + SkillUI,
// {
//     let _text = [T::title(), T::label()].join("\n");
//     panel.with_child((
//         assets.image_node(T::tile_index()),
//         // ShowPopupOnMouseOver {
//         //     text,
//         //     image: Some(assets.image_node(T::tile_index())),
//         // },
//     ));
// }
