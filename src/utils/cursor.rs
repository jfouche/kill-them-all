use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

pub fn grab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(window) = primary_window.get_single_mut() {
        set_grab_cursor(window, true);
    }
}

pub fn ungrab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(window) = primary_window.get_single_mut() {
        set_grab_cursor(window, false);
    }
}

pub fn set_grab_cursor(mut window: Mut<Window>, grab: bool) {
    if grab {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
    } else {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}
