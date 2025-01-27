use bevy::{prelude::*, window::PrimaryWindow};

/// see https://stackoverflow.com/questions/65396065/what-is-an-acceptable-approach-to-dragging-sprites-with-bevy-0-4
pub struct DndPlugin;

impl Plugin for DndPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<State>()
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, cursor_state)
            .add_systems(Update, (cursor_transform, draggable, hoverable))
            .add_systems(PostUpdate, (drag, drop));
    }
}

#[derive(Resource)]
struct State<'w, 's> {
    er_cursor_moved: EventReader<'w, 's, CursorMoved>,
}

#[derive(Component, Default)]
struct CursorState {
    cursor_world: Vec2,
    cursor_moved: bool,
}

#[derive(Component)]
#[require(Transform)]
struct Cursor;

#[derive(Component)]
struct Draggable;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Dragged;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Dropped;

#[derive(Component)]
struct Hoverable;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Hovered;

fn setup(mut commands: Commands) {
    commands.spawn(CursorState::default());
    commands.spawn(Cursor);
}

///
fn cursor_state(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q_cursor_state: Query<&mut CursorState>,
    q_camera: Query<&Transform, With<Camera>>,
    e_cursor_moved: Res<Events<CursorMoved>>,
    mut state: ResMut<State>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let event_cursor_screen = state.er_cursor_moved.latest(&e_cursor_moved);

    for mut cursor_state in &mut q_cursor_state {
        if let Some(event_cursor_screen) = event_cursor_screen {
            let cam_transform = q_camera.iter().last().unwrap();
            cursor_state.cursor_world =
                cursor_to_world(window, cam_transform, event_cursor_screen.position);

            cursor_state.cursor_moved = true;
        } else {
            cursor_state.cursor_moved = false;
        }
    }
}

fn cursor_transform(
    commands: &mut Commands,
    q_cursor_state: Query<&CursorState>,
    mut q_cursor: Query<(Entity, &mut Transform), With<Cursor>>,
) {
    let cursor_state = q_cursor_state.get_single().expect("CursorState");
    for (cursor_e, mut transform) in q_cursor.iter_mut() {
        transform.translation.x = cursor_state.cursor_world.x;
        transform.translation.y = cursor_state.cursor_world.y;
        commands.entity(cursor_e).remove_parent();
    }
}

fn hoverable(
    commands: &mut Commands,
    q_cursor_state: Query<&CursorState>,
    q_hoverable: Query<(Entity, &Transform, &Sprite), (With<Hoverable>, Without<Dragged>)>,
) {
    let cursor_state = q_cursor_state.get_single().expect("CursorState");
    if cursor_state.cursor_moved {
        for (entity, transform, sprite) in q_hoverable.iter() {
            let half_width = sprite.size.x / 2.0;
            let half_height = sprite.size.y / 2.0;

            if transform.translation.x - half_width < cursor_state.cursor_world.x
                && transform.translation.x + half_width > cursor_state.cursor_world.x
                && transform.translation.y - half_height < cursor_state.cursor_world.y
                && transform.translation.y + half_height > cursor_state.cursor_world.y
            {
                commands.entity(entity).insert(Hovered);
            } else {
                commands.entity(entity).remove::<Hovered>();
            }
        }
    }
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

fn draggable(
    commands: &mut Commands,
    i_mouse_button: Res<ButtonInput<MouseButton>>,
    q_pressed: Query<Entity, (With<Hovered>, With<Draggable>)>,
    q_released: Query<Entity, With<Dragged>>,
) {
    if i_mouse_button.just_pressed(MouseButton::Left) {
        if let Some(entity) = q_pressed.iter().next() {
            commands.entity(entity).insert(Dragged);
        }
    } else if i_mouse_button.just_released(MouseButton::Left) {
        for entity in q_released.iter() {
            commands.entity(entity).remove::<Dragged>().insert(Dropped);
        }
    }
}

fn drag(
    commands: &mut Commands,
    mut q_dragged: Query<(Entity, &mut Transform, &GlobalTransform), Added<Dragged>>,
    q_cursor: Query<(Entity, &GlobalTransform), With<Cursor>>,
) {
    if let Some((cursor_e, cursor_transform)) = q_cursor.iter().next() {
        for (entity, mut transform, global_transform) in q_dragged.iter_mut() {
            let global_pos = global_transform.translation() - cursor_transform.translation();

            commands.entity(entity).set_parent(cursor_e);

            transform.translation.x = global_pos.x;
            transform.translation.y = global_pos.y;
        }
    }
}

fn drop(
    commands: &mut Commands,
    mut q_dropped: Query<(Entity, &mut Transform, &GlobalTransform), Added<Dropped>>,
) {
    for (entity, mut transform, global_transform) in q_dropped.iter_mut() {
        let global_pos = global_transform.translation();

        transform.translation.x = global_pos.x;
        transform.translation.y = global_pos.y;

        commands.entity(entity).remove::<(Parent, Dropped)>();
    }
}
