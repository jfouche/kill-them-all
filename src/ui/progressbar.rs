use std::sync::{Arc, Mutex};

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (create_progress_bars, update_progress_bars));
}

/// The [ProgressBar] component should be nested with a [bevy::ui::node_bundles::NodeBundle]
#[derive(Debug, Clone, Component)]
pub struct ProgressBar {
    pub foreground: Color,
    pub background: Color,
    min: Arc<Mutex<f32>>,
    max: Arc<Mutex<f32>>,
    value: Arc<Mutex<f32>>,
}

impl ProgressBar {
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        ProgressBar {
            min: Arc::new(Mutex::new(min)),
            max: Arc::new(Mutex::new(max)),
            value: Arc::new(Mutex::new(value)),
            foreground: Color::WHITE,
            background: Color::BLACK,
        }
    }

    pub fn with_colors(mut self, background: Color, foreground: Color) -> Self {
        self.background = background;
        self.foreground = foreground;
        self
    }

    pub fn set_value(&mut self, value: f32) {
        if let Ok(mut store) = self.value.lock() {
            *store = value;
        }
    }
}

#[derive(Component)]
struct ProgressBarForeground {
    min: Arc<Mutex<f32>>,
    max: Arc<Mutex<f32>>,
    value: Arc<Mutex<f32>>,
}

impl ProgressBarForeground {
    fn new(data: &ProgressBar) -> Self {
        ProgressBarForeground {
            min: data.min.clone(),
            max: data.max.clone(),
            value: data.value.clone(),
        }
    }

    fn percent(&self) -> f32 {
        let min = self.min.lock().unwrap();
        let max = self.max.lock().unwrap();
        let value = self.value.lock().unwrap();
        *value / (*max - *min)
    }
}

fn create_progress_bars(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BackgroundColor, &ProgressBar), Added<ProgressBar>>,
) {
    for (entity, mut bkcolor, data) in query.iter_mut() {
        // Set background
        *bkcolor = data.background.into();

        // add foreground
        commands.entity(entity).with_children(|parent| {
            // foreground
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: data.foreground.into(),
                    ..default()
                },
                ProgressBarForeground::new(data),
            ));
        });
    }
}

fn update_progress_bars(mut child_query: Query<(&ProgressBarForeground, &mut Style)>) {
    for (data, mut style) in child_query.iter_mut() {
        style.width = Val::Percent(100.0 * data.percent());
    }
}
