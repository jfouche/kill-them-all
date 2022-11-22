use std::sync::{Arc, Mutex};

use bevy::prelude::*;

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_progress_bars)/* 
            .add_system(update_progress_bars) */;
    }
}

#[derive(Debug, Clone, Component)]
pub struct ProgressBarData {
    pub size: Size,
    pub foreground: Color,
    pub background: Color,
    pub min: f32,
    pub max: f32,
    value: Arc<Mutex<f32>>,
}

impl Default for ProgressBarData {
    fn default() -> Self {
        ProgressBarData {
            size: Size::new(Val::Px(100.0), Val::Px(16.0)),
            foreground: Color::WHITE,
            background: Color::BLACK,
            min: 0.0,
            max: 100.0,
            value: Arc::new(Mutex::new(66.0)),
        }
    }
}

#[derive(Component)]
struct ProgressBarForeground {
    pub percent_mutex: Arc<Mutex<f32>>,
}

#[derive(Bundle, Default)]
pub struct ProgressBarBundle {
    data: ProgressBarData,
    node: NodeBundle,
}

impl ProgressBarBundle {
    pub fn new(data: ProgressBarData) -> Self {
        let background_color = data.background.into();
        let size = data.size;
        ProgressBarBundle {
            data,
            node: NodeBundle {
                style: Style {
                    size,
                    border: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                background_color,
                ..Default::default()
            },
        }
    }
}

fn create_progress_bars(
    mut commands: Commands,
    mut query: Query<(Entity, &ProgressBarData), Added<ProgressBarData>>,
) {
    for (entity, data) in query.iter_mut() {
        warn!("create_progress_bars");
        commands
            .entity(entity)
            .insert(Name::new("ProgressBar"))
            // background
            .insert(NodeBundle {
                style: Style {
                    size: data.size,
                    border: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                background_color: data.background.into(),
                ..Default::default()
            })
            .with_children(|background| {
                // foreground
                background.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(66.0), Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: data.foreground.into(),
                    ..default()
                });
            });
    }
}

fn update_progress_bars(mut query: Query<(&ProgressBarData, &mut Style)>) {
    // for (data, mut style) in query.iter_mut() {
    //     let percent = *(data.value) / (data.max - data.min);
    //     style.size.width = Val::Percent(percent);
    // }
}
