use crate::prelude::*;

mod progressbar;

pub use progressbar::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bevy_ui_navigation::systems::default_keyboard_input);
    }
}
