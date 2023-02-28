use bevy_ui_navigation::prelude::*;

use crate::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin)
            .register_type::<Money>()
            .add_system(print_nav_events.after(NavRequestSystem));
    }
}

fn print_nav_events(mut events: EventReader<NavEvent>) {
    for event in events.iter() {
        warn!("{:?}", event);
    }
}
