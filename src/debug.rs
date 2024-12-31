#![allow(unused)]

use crate::{components::*, schedule::*, utils::cursor::set_grab_cursor};
use bevy::{
    dev_tools::{fps_overlay::*, states::log_transitions, ui_debug_overlay::*},
    input::common_conditions::input_just_pressed,
    prelude::*,
    window::PrimaryWindow,
};
use bevy_rapier2d::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // DebugUiPlugin,
            FpsOverlayPlugin::default(),
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
            bevy_inspector_egui::quick::FilterQueryInspectorPlugin::<With<Player>>::default(),
            bevy_inspector_egui::quick::FilterQueryInspectorPlugin::<With<Monster>>::default(),
            bevy_rapier2d::render::RapierDebugRenderPlugin::default(),
        ))
        .add_systems(
            Update,
            (
                toggle_grab,
                log_transitions::<GameState>,
                log_transitions::<InGameState>,
                // display_collision_events.in_set(GameRunningSet::EntityUpdate),
                toggle_debug_ui.run_if(input_just_pressed(KeyCode::Backquote)),
            ),
        );
    }
}

fn toggle_grab(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(window) = primary_window.get_single_mut() {
        if keys.just_pressed(KeyCode::KeyG) {
            match window.cursor_options.grab_mode {
                bevy::window::CursorGrabMode::None => {
                    set_grab_cursor(window, true);
                }
                _ => {
                    set_grab_cursor(window, false);
                }
            }
        }
    }
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
