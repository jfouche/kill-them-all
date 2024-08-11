use crate::{cursor::set_grab_cursor, schedule::*};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
            bevy_rapier2d::render::RapierDebugRenderPlugin::default(),
        ))
        .add_systems(
            Update,
            (
                toggle_grab,
                state_transition::<GameState>,
                state_transition::<InGameState>,
                display_collision_events.in_set(GameRunningSet::EntityUpdate),
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
            match window.cursor.grab_mode {
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

fn state_transition<S: States>(mut events: EventReader<StateTransitionEvent<S>>) {
    for event in events.read() {
        let name = std::any::type_name::<S>();
        info!("{name} : {event:?}");
    }
}

fn display_collision_events(mut collisions: EventReader<CollisionEvent>, names: Query<DebugName>) {
    let get_name = |e| {
        names
            .get(e)
            .map(|dn| format!("{dn:?}"))
            .unwrap_or(format!("{e:?}"))
    };

    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(e1, e2, flag) => {
                let n1 = get_name(*e1);
                let n2 = get_name(*e2);
                info!("CollisionEvent::Started({n1}, {n2}, {flag:?})");
            }
            CollisionEvent::Stopped(e1, e2, flag) => {
                let n1 = get_name(*e1);
                let n2 = get_name(*e2);
                info!("CollisionEvent::Stopped({n1}, {n2}, {flag:?})");
            }
        }
    }
}
