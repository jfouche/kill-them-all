use bevy::{prelude::*, window::PrimaryWindow};

pub struct DndPlugin;

impl Plugin for DndPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DraggedEntity>()
            .init_resource::<CursorState>()
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, cursor_state)
            .add_systems(Update, move_cursor);
    }
}

#[derive(Resource, Default)]

struct CursorState {
    pos: Vec2,
    world_pos: Vec2,
}

#[derive(Component, Reflect)]
#[require(
    Name(|| Name::new("DndCursor")),
    DraggedEntity,
    Node(|| Node {
        display: Display::Block,
        position_type: PositionType::Absolute,
        ..Default::default()
    }),
    ImageNode,
    ZIndex(|| ZIndex(1))
)]
pub struct DndCursor;

#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub struct DraggedEntity(pub Option<Entity>);

fn setup(mut commands: Commands) {
    commands.spawn(DndCursor);
}

fn cursor_state(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cursor_state: ResMut<CursorState>,
    cameras: Query<&Transform, With<Camera>>,
    mut events: EventReader<CursorMoved>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok(camera_transform) = cameras.get_single() else {
        return;
    };
    for event in events.read() {
        cursor_state.pos = event.position;
        cursor_state.world_pos = cursor_to_world(window, camera_transform, event.position);
    }
}

fn move_cursor(cursor_state: Res<CursorState>, mut cursor: Single<&mut Node, With<DndCursor>>) {
    cursor.left = Val::Px(cursor_state.pos.x);
    cursor.top = Val::Px(cursor_state.pos.y);
}

fn cursor_to_world(window: &Window, cam_transform: &Transform, cursor_pos: Vec2) -> Vec2 {
    // get the size of the window
    let size = Vec2::new(window.width() as f32, window.height() as f32);

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let screen_pos = cursor_pos - size / 2.0;

    // apply the camera transform
    let out = cam_transform.compute_matrix() * screen_pos.extend(0.0).extend(1.0);
    out.xy()
}
