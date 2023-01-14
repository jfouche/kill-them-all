use crate::prelude::*;
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_world)
            .add_system(camera_fit_inside_current_level)
            .add_system(test);
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

fn init_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(LdtkWorldBundle {
            ldtk_handle: asset_server.load("kill-them-all.ldtk"),
            ..Default::default()
        })
        .insert(Name::new("World"));
}

const ASPECT_RATIO: f32 = 16. / 9.;

fn camera_fit_inside_current_level(
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;

                    orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                    orthographic_projection.bottom = 0.;
                    orthographic_projection.left = 0.;
                    if level_ratio > ASPECT_RATIO {
                        // level is wider than the screen
                        orthographic_projection.top = (level.px_hei as f32 / 9.).round() * 9.;
                        orthographic_projection.right = orthographic_projection.top * ASPECT_RATIO;
                        camera_transform.translation.x = (player_translation.x
                            - level_transform.translation.x
                            - orthographic_projection.right / 2.)
                            .clamp(0., level.px_wid as f32 - orthographic_projection.right);
                        camera_transform.translation.y = 0.;
                    } else {
                        // level is taller than the screen
                        orthographic_projection.right = (level.px_wid as f32 / 16.).round() * 16.;
                        orthographic_projection.top = orthographic_projection.right / ASPECT_RATIO;
                        camera_transform.translation.y = (player_translation.y
                            - level_transform.translation.y
                            - orthographic_projection.top / 2.)
                            .clamp(0., level.px_hei as f32 - orthographic_projection.top);
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }
}

fn test(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    level_assets: Res<Assets<LdtkLevel>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        warn!("test 1");
        for (ldtk_entity, level_handle) in level_query.iter() {
            warn!("test 2");
            if let Some(level) = level_assets.get(level_handle) {
                let level = &level.level;
                if let Some(layer_instances) = &level.layer_instances {
                    let z = layer_instances.len() as f32;
                    commands.entity(ldtk_entity).with_children(|layer| {
                        layer
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::BLACK,
                                    custom_size: Some(Vec2::new(5.0, 5.0)),
                                    ..Default::default()
                                },
                                transform: Transform::from_translation(Vec3::new(0., 0., z)),
                                ..Default::default()
                            })
                            .insert(Name::new("==TEST=="));
                    });
                }
            }
        }
    }
}
