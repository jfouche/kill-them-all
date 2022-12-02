use std::sync::{Arc, Mutex};

use bevy::prelude::*;

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_progress_bars)
            .add_system(update_progress_bars);
    }
}

#[derive(Debug, Clone, Component)]
pub struct ProgressBarData {
    pub size: Size,
    pub foreground: Color,
    pub background: Color,
    percent: Arc<Mutex<f32>>,
}

impl Default for ProgressBarData {
    fn default() -> Self {
        ProgressBarData {
            size: Size::new(Val::Px(100.0), Val::Px(16.0)),
            foreground: Color::WHITE,
            background: Color::BLACK,
            percent: Arc::new(Mutex::new(0.0)),
        }
    }
}

impl ProgressBarData {
    pub fn from_size(size: Size) -> Self {
        ProgressBarData {
            size,
            ..Default::default()
        }
    }

    pub fn with_colors(mut self, background: Color, foreground: Color) -> Self {
        self.background = background;
        self.foreground = foreground;
        self
    }

    pub fn set_percent(&mut self, value: f32) {
        if let Ok(mut store) = self.percent.lock() {
            *store = value;
        }
    }
}

#[derive(Component)]
struct ProgressBarForeground {
    pub percent: Arc<Mutex<f32>>,
}

#[derive(Bundle)]
pub struct ProgressBarBundle {
    data: ProgressBarData,
    node: NodeBundle,
}

pub trait Percent {
    /// Percent : 1.0 is 1%, 100.0 is 100%
    fn percent(&self) -> f32;
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
                    margin: UiRect::all(Val::Px(3.0)),
                    border: UiRect::all(Val::Px(2.0)),
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
    query: Query<(Entity, &ProgressBarData), Added<ProgressBarData>>,
) {
    for (entity, data) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            // foreground
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(0.0), Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: data.foreground.into(),
                    ..default()
                })
                .insert(ProgressBarForeground {
                    percent: data.percent.clone(),
                });
        });
    }
}

fn update_progress_bars(mut child_query: Query<(&ProgressBarForeground, &mut Style)>) {
    for (data, mut style) in child_query.iter_mut() {
        let value = *data.percent.lock().unwrap();
        style.size.width = Val::Percent(value);
    }
}
