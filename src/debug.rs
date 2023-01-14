use crate::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
            .register_type::<Weapon>()
            .register_type::<MovementSpeed>()
            .register_type::<Life>()
            .register_type::<AttackSpeed>()
            .register_type::<Money>();
    }
}
