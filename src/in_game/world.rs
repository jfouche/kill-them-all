use crate::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_world);
    }
}

const WORLD_WIDTH: usize = 36;
const WORLD_HEIGHT: usize = 24;

const WORLD_WIDTH_F32: f32 = WORLD_WIDTH as f32;
const WORLD_HEIGHT_F32: f32 = WORLD_HEIGHT as f32;

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
                    custom_size: Some(Vec2::new(WORLD_WIDTH_F32, WORLD_HEIGHT_F32)),
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
                    custom_size: Some(Vec2::new(WORLD_WIDTH_F32, BORDER)),
                    color: Color::NONE,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., WORLD_HEIGHT_F32 / 2. + BORDER / 2., 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(WORLD_WIDTH_F32 / 2., BORDER / 2.),
        }
    }

    fn right() -> Self {
        Border {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BORDER, WORLD_HEIGHT_F32)),
                    color: Color::NONE,
                    ..Default::default()
                },
                transform: Transform::from_xyz(WORLD_WIDTH_F32 / 2. + BORDER / 2., 0.0, 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(BORDER / 2., WORLD_HEIGHT_F32 / 2.),
        }
    }

    fn bottom() -> Self {
        Border {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(WORLD_WIDTH_F32, BORDER)),
                    color: Color::NONE,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., -WORLD_HEIGHT_F32 / 2. - BORDER / 2., 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(WORLD_WIDTH_F32 / 2., BORDER / 2.),
        }
    }

    fn left() -> Self {
        Border {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(WORLD_WIDTH_F32, BORDER)),
                    color: Color::NONE,
                    ..Default::default()
                },
                transform: Transform::from_xyz(-WORLD_WIDTH_F32 / 2. - BORDER / 2., 0.0, 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(BORDER / 2., WORLD_HEIGHT_F32 / 2.),
        }
    }
}

fn init_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn(WorldBundle::default())
        .insert(Name::new("World"))
        .with_children(|world| {
            const TOP: usize = 0;
            const BOTTOM: usize = WORLD_HEIGHT - 1;
            const LEFT: usize = 0;
            const RIGHT: usize = WORLD_HEIGHT - 1;
            let texture_handle = asset_server.load("background/TilesetFloor.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 22, 28, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            for row in 0..WORLD_HEIGHT {
                for col in 0..WORLD_WIDTH {
                    let index = match (row, col) {
                        (TOP, LEFT) => 0,
                        (TOP, RIGHT) => 3,
                        (BOTTOM, LEFT) => 44,
                        (BOTTOM, RIGHT) => 46,
                        (TOP, _) => 1,
                        (BOTTOM, _) => 45,
                        (_, LEFT) => 22,
                        (_, RIGHT) => 24,
                        (_, _) => 23,
                    };
                    let x = col as f32;
                    let y = row as f32;

                    world
                        .spawn(SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                custom_size: Some(Vec2::new(1.0, 1.0)),
                                index,
                                ..Default::default()
                            },
                            texture_atlas: texture_atlas_handle.clone(),
                            transform: Transform::from_xyz(x, y, 10.),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile"));
                }
            }
        });
    // .add_children(|builder| {
    //     builder.spawn(Border::top());
    //     builder.spawn(Border::right());
    //     builder.spawn(Border::bottom());
    //     builder.spawn(Border::left());
    // });
}
