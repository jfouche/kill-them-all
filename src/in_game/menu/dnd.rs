use crate::components::item::Item;
use bevy::{color::palettes::css, ecs::query::QueryFilter, prelude::*, window::PrimaryWindow};
use std::marker::PhantomData;

pub struct DndPlugin;

impl Plugin for DndPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DraggedEntity>()
            .init_resource::<CursorState>()
            .add_systems(Startup, spawn_dnd_cursor)
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

fn spawn_dnd_cursor(mut commands: Commands) {
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

pub struct ShowBorderOnDrag<F = With<Item>> {
    on_enter: Observer,
    on_leave: Observer,
    _phantom: PhantomData<F>,
}

impl<F> ShowBorderOnDrag<F>
where
    F: QueryFilter + 'static,
{
    pub fn new() -> Self {
        Self {
            on_enter: Observer::new(show_borders_on_drag_enter_item::<F>),
            on_leave: Observer::new(hide_borders_on_drag_leave_item),
            _phantom: PhantomData::<F>,
        }
    }

    pub fn watch_entity(&mut self, entity: Entity) {
        self.on_enter.watch_entity(entity);
        self.on_leave.watch_entity(entity);
    }

    pub fn spawn(self, commands: &mut Commands) {
        commands.spawn(self.on_enter);
        commands.spawn(self.on_leave);
    }
}

fn show_borders_on_drag_enter_item<F>(
    trigger: Trigger<Pointer<DragEnter>>,
    mut borders: Query<&mut BorderColor>,
    items: Query<(), F>,
    cursor: Single<&DraggedEntity, With<DndCursor>>,
) where
    F: QueryFilter,
{
    warn!("show_borders_on_drag_enter_item({})", trigger.entity());
    if let Some(item_entity) = ***cursor {
        if items.get(item_entity).is_ok() {
            if let Ok(mut border_color) = borders.get_mut(trigger.entity()) {
                border_color.0 = css::DARK_ORANGE.into();
            }
        }
    }
}

fn hide_borders_on_drag_leave_item(
    trigger: Trigger<Pointer<DragLeave>>,
    mut borders: Query<&mut BorderColor>,
) {
    warn!("hide_borders_on_drag_leave_item({})", trigger.entity());
    if let Ok(mut border_color) = borders.get_mut(trigger.entity()) {
        border_color.0 = Srgba::NONE.into();
    };
}
