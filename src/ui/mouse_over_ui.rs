use bevy::{prelude::*, ui::RelativeCursorPosition};

pub struct MouseOverUiPlugin;

impl Plugin for MouseOverUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseOverUi>()
            .add_systems(PreUpdate, update_mouse_handling);
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct MouseOverUi(bool);

/// Component to add to [Node] in order to inform that the mouse is over it.
///
///  Use it alongside the [mouse_not_over_ui] run condition if you want to
/// block mouse click on world while on a UI.
///
/// ex:
/// ```
/// #[derive(Component)]
/// #[(require(Node{...}, CaptureMouse))]
/// struct MyPanel;
///
/// fn my_plugin(app: &mut App) {
///     app.add_systems(Update, move_player.run_if(mouse_not_over_ui));
/// }
/// ```
#[derive(Component, Default)]
#[require(RelativeCursorPosition)]
pub struct CaptureMouse;

/// Run condition that will run false if the mouse is over a [CaptureMouse] node.
pub fn mouse_not_over_ui(value: Res<MouseOverUi>) -> bool {
    !**value
}

fn update_mouse_handling(
    nodes: Query<&RelativeCursorPosition, With<CaptureMouse>>,
    mut mouse_over_ui: ResMut<MouseOverUi>,
) {
    **mouse_over_ui = nodes.iter().any(|pos| pos.mouse_over());
}
