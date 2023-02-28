use crate::prelude::*;

mod progressbar;

use bevy_ui_navigation::{systems::InputMapping, DefaultNavigationPlugins};
pub use progressbar::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultNavigationPlugins)
            .add_plugin(ProgressBarPlugin)
            .add_startup_system(init_menu_keyboard);
    }
}

fn init_menu_keyboard(mut input: ResMut<InputMapping>) {
    warn!("init_menu_keyboard");
    input.keyboard_navigation = true;
    input.key_down = KeyCode::Down;
    input.key_down_alt = KeyCode::Numpad2;
    input.key_up = KeyCode::Up;
    input.key_up_alt = KeyCode::Numpad8;
    input.key_left = KeyCode::Left;
    input.key_left_alt = KeyCode::Numpad4;
    input.key_right = KeyCode::Right;
    input.key_right_alt = KeyCode::Numpad6;
    input.key_action = KeyCode::Space;
}
