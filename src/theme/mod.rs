// Unused utilities may trigger this lints undesirably.
#![allow(dead_code)]

pub mod interaction;
pub mod palette;
pub mod widget;

pub fn theme_plugin(app: &mut bevy::app::App) {
    // app.add_plugins(interaction::plugin);
}
