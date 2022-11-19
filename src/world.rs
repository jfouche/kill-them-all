use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_world);
    }
}

const WORLD_WIDTH: f32 = 35.0;
const WORLD_HEIGH: f32 = 25.0;

const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
const BORDER: f32 = 1.0;

#[derive(Bundle)]
struct WorldBundle {
    #[bundle]
    sprite: SpriteBundle,
}

impl WorldBundle {
    fn default() -> Self {
        WorldBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(WORLD_WIDTH, WORLD_HEIGH)),
                    color: Color::rgb(0.6, 0.6, 0.6),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
struct Border {
    #[bundle]
    sprite: SpriteBundle,
    collider: Collider,
}

impl Border {
    fn top() -> Self {
        Border {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(WORLD_WIDTH, BORDER)),
                    color: TRANSPARENT,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., WORLD_HEIGH / 2. + BORDER / 2., 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(WORLD_WIDTH / 2., BORDER / 2.),
        }
    }

    fn right() -> Self {
        Border {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BORDER, WORLD_HEIGH)),
                    color: TRANSPARENT,
                    ..Default::default()
                },
                transform: Transform::from_xyz(WORLD_WIDTH / 2. + BORDER / 2., 0.0, 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(BORDER / 2., WORLD_HEIGH / 2.),
        }
    }

    fn bottom() -> Self {
        Border {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(WORLD_WIDTH, BORDER)),
                    color: TRANSPARENT,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., -WORLD_HEIGH / 2. - BORDER / 2., 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(WORLD_WIDTH / 2., BORDER / 2.),
        }
    }

    fn left() -> Self {
        Border {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(WORLD_WIDTH, BORDER)),
                    color: TRANSPARENT,
                    ..Default::default()
                },
                transform: Transform::from_xyz(-WORLD_WIDTH / 2. - BORDER / 2., 0.0, 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(BORDER / 2., WORLD_HEIGH / 2.),
        }
    }
}

fn init_world(mut commands: Commands) {
    commands
        .spawn(WorldBundle::default())
        .insert(Name::new("World"))
        .add_children(|builder| {
            builder.spawn(Border::top());
            builder.spawn(Border::right());
            builder.spawn(Border::bottom());
            builder.spawn(Border::left());
        });
}
