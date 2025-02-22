#![allow(unused)]

use crate::{
    components::{monster::Monster, player::Player},
    schedule::*,
};
use bevy::{
    dev_tools::{fps_overlay::*, states::log_transitions, ui_debug_overlay::*},
    input::common_conditions::input_just_pressed,
    prelude::*,
    window::PrimaryWindow,
};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiPlugin},
    bevy_inspector::{self, guess_entity_name, hierarchy::SelectedEntities, EntityFilter, Filter},
    egui, DefaultInspectorConfigPlugin,
};
use bevy_rapier2d::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DebugUiPlugin,
            FpsOverlayPlugin::default(),
            EguiPlugin,
            DefaultInspectorConfigPlugin,
            bevy_rapier2d::render::RapierDebugRenderPlugin::default(),
        ))
        .add_systems(
            Update,
            (
                inspector_ui,
                log_transitions::<GameState>,
                log_transitions::<InGameState>,
                // display_collision_events.in_set(GameRunningSet::EntityUpdate),
                toggle_debug_ui.run_if(input_just_pressed(KeyCode::Backquote)),
            ),
        );
    }
}

fn inspector_ui(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();
    egui::Window::new("World").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            let filter = Filter::<(Without<Parent>, Without<Observer>)>::from_ui_fuzzy(
                ui,
                egui::Id::new("KTE DEBUG INSPECTOR FILTER"),
            );
            bevy_inspector::ui_for_entities_filtered(world, ui, true, &filter);
            ui.allocate_space(ui.available_size());
        });
    });
}

fn display_collision_events(
    mut collisions: EventReader<CollisionEvent>,
    names: Query<NameOrEntity>,
) {
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
