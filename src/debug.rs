#![allow(unused)]

use crate::{
    components::{
        affix::{IncreaseAreaOfEffect, PierceChance},
        equipment::Amulet,
        inventory::{
            AddToInventoryEvent, InventoryChanged, PlayerEquipmentChanged, TakeDroppedItemEvent,
        },
        item::{DroppedItem, ItemAssets, ItemDescriptor, ItemRarity, ItemSpawner},
        monster::Monster,
        orb::{Orb, OrbProvider},
        player::{EquipSkillBookEvent, Player, PlayerBooks, RemoveSkillBookEvent},
        skills::{
            death_aura::{DeathAura, DeathAuraBook},
            AssociatedSkill, SkillBook,
        },
        world_map::{ProceduralWorldMap, WorldMapConfig, LAYER_ITEM},
    },
    in_game::{item_plugin::take_dropped_item, life_bar_plugin::LifeBar},
    schedule::*,
};
use bevy::{
    dev_tools::{fps_overlay::*, states::log_transitions},
    ecs::entity::Entities,
    input::common_conditions::{input_just_pressed, input_just_released},
    math::vec2,
    prelude::*,
    time::common_conditions::on_timer,
    window::PrimaryWindow,
};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiContextPass, EguiGlobalSettings, EguiPlugin},
    bevy_inspector::{self, guess_entity_name, hierarchy::SelectedEntities, EntityFilter, Filter},
    egui,
    quick::WorldInspectorPlugin,
    DefaultInspectorConfigPlugin,
};
use bevy_rapier2d::prelude::*;
use std::time::Duration;

#[derive(Resource, Deref, DerefMut)]
struct DebugMode(bool);

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            FpsOverlayPlugin::default(),
            DefaultInspectorConfigPlugin,
            bevy_rapier2d::render::RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new().run_if(debug_is_active),
        ))
        .init_resource::<UiDebugOptions>()
        .insert_resource(DebugMode(true))
        .add_systems(EguiContextPass, inspector_ui.run_if(debug_is_active))
        .add_systems(Startup, configure_egui)
        .add_systems(
            Update,
            (
                toggle_debug_mode.run_if(input_just_released(KeyCode::KeyD)),
                (count_entities, show_player_pos).run_if(input_just_released(KeyCode::KeyL)),
                (
                    log_transitions::<GameState>,
                    log_transitions::<InGameState>,
                    // show_key_pressed,
                    // display_collision_events.in_set(GameRunningSet::EntityUpdate),
                    // show_map_axes.run_if(resource_exists::<ProceduralWorldMap>),
                )
                    .run_if(debug_is_active),
                toggle_debug_ui.run_if(input_just_pressed(KeyCode::Backquote)),
            ),
        )
        // .add_systems(Last, debug_death_aura_post)
        .add_observer(init_player)
        .add_observer(|_: Trigger<InventoryChanged>| info!("InventoryChanged"))
        .add_observer(|_: Trigger<PlayerEquipmentChanged>| info!("PlayerEquipmentChanged"))
        .add_observer(|_: Trigger<EquipSkillBookEvent>| info!("EquipSkillBookEvent"))
        .add_observer(|_: Trigger<RemoveSkillBookEvent>| info!("RemoveSkillBookEvent"));
    }
}

fn debug_is_active(debug: Res<DebugMode>) -> bool {
    **debug
}

fn toggle_debug_mode(mut mode: ResMut<DebugMode>) {
    **mode = !**mode;
}

fn configure_egui(mut settings: ResMut<EguiGlobalSettings>) {
    settings.enable_absorb_bevy_input_system = true;
}

fn inspector_ui(world: &mut World) {
    let Ok(mut egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .cloned()
    else {
        return;
    };
    egui::Window::new("World").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            let filter =
                Filter::<(
                    Without<ChildOf>,
                    Without<Observer>,
                    Without<Monster>,
                    Without<LifeBar>,
                )>::from_ui_fuzzy(ui, egui::Id::new("KTE DEBUG INSPECTOR FILTER"));
            bevy_inspector::ui_for_entities_filtered(world, ui, true, &filter);
            ui.allocate_space(ui.available_size());
        });
    });
}

fn display_collision_events(
    mut collisions: EventReader<CollisionEvent>,
    names: Query<NameOrEntity>,
    debug: Res<DebugMode>,
) {
    if !**debug {
        return;
    }
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(e1, e2, flag) => {
                let n1 = names.get(*e1).unwrap();
                let n2 = names.get(*e2).unwrap();
                info!("CollisionEvent::Started({n1}, {n2}, {flag:?})");
            }
            CollisionEvent::Stopped(e1, e2, flag) => {
                let n1 = names.get(*e1).unwrap();
                let n2 = names.get(*e2).unwrap();
                info!("CollisionEvent::Stopped({n1}, {n2}, {flag:?})");
            }
        }
    }
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn count_entities(entities: &Entities) {
    info!("count_entities() : {}", entities.len());
}

fn show_key_pressed(inputs: Res<ButtonInput<KeyCode>>) {
    let key_pressed = inputs
        .get_just_pressed()
        .map(|k| format!("{k:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    if !key_pressed.is_empty() {
        info!("show_key_pressed() : {key_pressed}");
    }
}

fn show_player_pos(players: Query<&Transform, With<Player>>, world_map: Res<ProceduralWorldMap>) {
    if let Ok(transform) = players.single() {
        let player_translation = transform.translation.xy();
        let player_pos = world_map.world_to_pos(player_translation);
        let chunk_pos = world_map.chunk_pos(player_translation);
        info!("Player - map pos: {player_pos}, chunk pos: {chunk_pos}, world translation: {player_translation}");
    }
}

fn show_map_axes(mut gizmos: Gizmos, world_map: Res<ProceduralWorldMap>) {
    let zero = world_map.pos_to_world(0, 0);
    let one = world_map.pos_to_world(1, 1);
    gizmos.line_2d(zero, zero + one, Color::srgba_u8(20, 172, 121, 255));
}

fn init_player(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    let player = trigger.target();
    let mut rng = rand::rng();

    let orb = commands.spawn(Orb::Transmutation).id();
    commands.trigger(AddToInventoryEvent::new(orb));

    let orb = commands.spawn(Orb::Alteration).id();
    commands.trigger(AddToInventoryEvent::new(orb));

    let orb = commands.spawn(Orb::Regal).id();
    commands.trigger(AddToInventoryEvent::new(orb));

    let orb = commands.spawn(Orb::Chaos).id();
    commands.trigger(AddToInventoryEvent::new(orb));

    let spawner = ItemSpawner::new(1, &mut rng);
    let amulet = spawner.spawn::<Amulet>(&mut commands, &mut rng);
    commands.trigger(AddToInventoryEvent::new(amulet));
}

fn debug_death_aura(q: Query<&Transform, With<DeathAura>>, mut save: Local<Vec2>) {
    for &transform in &q {
        let pos = transform.translation.xy();
        if *save != pos {
            warn!("DeathAura transform: {}", transform.translation.xy());
            *save = pos;
        }
    }
}

fn debug_death_aura_post(q: Query<&Transform, With<DeathAura>>, mut save: Local<Vec2>) {
    for &transform in &q {
        let pos = transform.translation.xy();
        if *save != pos {
            warn!(
                "DeathAura transform PostUpdate: {}",
                transform.translation.xy()
            );
            *save = pos;
        }
    }
}

fn equip_death_aura(mut commands: Commands, players: Query<Entity, With<Player>>) {
    let player = players.single().unwrap();
    let book = commands
        .spawn((Name::new("TEST BOOK"), SkillBook, ChildOf(player)))
        .id();
    let skill = commands
        .spawn((Name::new("TEST DEATH AURA"), DeathAura, ChildOf(player)))
        .id();
    commands.entity(book).insert(AssociatedSkill(skill));
}
