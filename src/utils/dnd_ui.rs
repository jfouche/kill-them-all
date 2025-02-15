use bevy::{prelude::*, window::PrimaryWindow};

/// see <https://stackoverflow.com/questions/65396065/what-is-an-acceptable-approach-to-dragging-sprites-with-bevy-0-4>
pub struct DndPlugin;

impl Plugin for DndPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CursorState>()
            // .init_resource::<State>()
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, cursor_state)
            .add_systems(Update, (move_cursor /* , draggable*/, hoverable))
            // .add_systems(PostUpdate, (drag, drop))
            .add_observer(init_dragged)
            .add_observer(init_draggable);
    }
}

#[derive(Component, Default, Reflect)]
#[require(
    Name(|| Name::new("CursorState"))
)]
struct CursorState {
    moved: bool,
    pos: Vec2,
    world_pos: Vec2,
}

#[derive(Component)]
#[require(
    Name(|| Name::new("Cursor")),
    Node(|| Node {
        display: Display::Block,
        position_type: PositionType::Absolute,
        ..Default::default()
    }),
    ZIndex(|| ZIndex(1))
)]
pub struct Cursor;

/// Component which allow an entity to be dragged
#[derive(Component)]
pub struct Draggable;

/// Component present when a [Draggable] entity is dragged
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Dragged;

// #[derive(Component)]
// #[component(storage = "SparseSet")]
// struct DraggedParent(Option<Entity>);

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Dropped;

#[derive(Component)]
pub struct Hoverable;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Hovered;

fn setup(mut commands: Commands) {
    commands.spawn(CursorState::default());
    commands.spawn(Cursor);
}

fn init_dragged(
    trigger: Trigger<OnAdd, Dragged>,
    mut commands: Commands,
    cursor: Single<Entity, With<Cursor>>,
    // mut items: Query<(&ImageNode, &mut Visibility)>,
    // parents: Query<&Parent>,
) {
    warn!("init_dragged({}", trigger.entity());
    // let parent = parents.get(trigger.entity()).map(|p| **p).ok();
    commands.entity(trigger.entity()).set_parent(*cursor);
    // if let Ok((img, mut visibility)) = items.get_mut(trigger.entity()) {
    //     *visibility = Visibility::Hidden;
    //     cursor.0.replace(trigger.entity());
    //     *cursor.1 = img.clone();
    // }
}

fn init_draggable(trigger: Trigger<OnAdd, Draggable>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(start_drag)
        .observe(end_drag);
}

fn start_drag(
    trigger: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    cursor: Single<Entity, With<Cursor>>,
) {
    warn!("start_drag");
    commands
        .entity(trigger.entity())
        .insert(Dragged)
        .set_parent(*cursor);
}

fn end_drag(trigger: Trigger<Pointer<DragDrop>>, mut commands: Commands) {
    warn!("start_drag");
    commands
        .entity(trigger.entity())
        .remove::<Dragged>()
        .insert(Dropped);
}

///
fn cursor_state(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cursor_state: Single<&mut CursorState>,
    cameras: Query<&Transform, With<Camera>>,
    mut events: EventReader<CursorMoved>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok(camera_transform) = cameras.get_single() else {
        return;
    };
    cursor_state.moved = false;
    for event in events.read() {
        cursor_state.pos = event.position;
        cursor_state.world_pos = cursor_to_world(window, camera_transform, event.position);
        cursor_state.moved = true;
    }
}

fn move_cursor(cursor_state: Single<&CursorState>, mut cursor: Single<&mut Node, With<Cursor>>) {
    cursor.left = Val::Px(cursor_state.pos.x);
    cursor.top = Val::Px(cursor_state.pos.y);
}

fn hoverable(
    mut commands: Commands,
    cursor_state: Single<&CursorState>,
    hoverables: Query<(Entity, &Transform, &Sprite), (With<Hoverable>, Without<Dragged>)>,
    assets: Res<Assets<Image>>,
) {
    if cursor_state.moved {
        for (entity, transform, sprite) in hoverables.iter() {
            if let Some(image_dimensions) =
                assets.get(&sprite.image).map(|img| img.size().as_vec2())
            {
                let scaled_image_dimension = image_dimensions * transform.scale.xy();
                let half_width = scaled_image_dimension.x / 2.0;
                let half_height = scaled_image_dimension.y / 2.0;

                if transform.translation.x - half_width < cursor_state.pos.x
                    && transform.translation.x + half_width > cursor_state.pos.x
                    && transform.translation.y - half_height < cursor_state.pos.y
                    && transform.translation.y + half_height > cursor_state.pos.y
                {
                    warn!("hoverable: {entity} hovered");
                    commands.entity(entity).insert(Hovered);
                } else {
                    warn!("hoverable: {entity} not hovered anymore");
                    commands.entity(entity).remove::<Hovered>();
                }
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

// fn draggable(
//     mut commands: Commands,
//     mouse_button: Res<ButtonInput<MouseButton>>,
//     q_pressed: Query<Entity, (With<Hovered>, With<Draggable>)>,
//     q_released: Query<Entity, With<Dragged>>,
// ) {
//     if mouse_button.just_pressed(MouseButton::Left) {
//         if let Some(entity) = q_pressed.iter().next() {
//             warn!("draggable: {entity} dragged");
//             commands.entity(entity).insert(Dragged);
//         }
//     } else if mouse_button.just_released(MouseButton::Left) {
//         for entity in q_released.iter() {
//             warn!("draggable: {entity} dropped");
//             commands.entity(entity).remove::<Dragged>().insert(Dropped);
//         }
//     }
// }

// fn drag(
//     mut commands: Commands,
//     mut q_dragged: Query<(Entity, &mut Transform, &GlobalTransform), Added<Dragged>>,
//     q_cursor: Query<(Entity, &GlobalTransform), With<Cursor>>,
// ) {
//     if let Some((cursor_e, cursor_transform)) = q_cursor.iter().next() {
//         for (entity, mut transform, global_transform) in q_dragged.iter_mut() {
//             warn!("drag {entity}");
//             let global_pos = global_transform.translation() - cursor_transform.translation();

//             commands.entity(entity).set_parent(cursor_e);

//             transform.translation.x = global_pos.x;
//             transform.translation.y = global_pos.y;
//         }
//     }
// }

// fn drop(
//     mut commands: Commands,
//     mut droppeds: Query<(Entity, &mut Transform, &GlobalTransform), Added<Dropped>>,
// ) {
//     for (entity, mut transform, global_transform) in &mut droppeds {
//         warn!("drop");
//         let global_pos = global_transform.translation();

//         transform.translation.x = global_pos.x;
//         transform.translation.y = global_pos.y;

//         commands.entity(entity).remove::<(Parent, Dropped)>();
//     }
// }
