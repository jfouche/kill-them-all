use std::sync::{Arc, Mutex};

use bevy::prelude::*;

pub fn progressbar_plugin(app: &mut App) {
    app.add_observer(create_progress_bars)
        .add_systems(Update, update_progress_bars);
}

/// The [ProgressBar] component should be nested with a [bevy::ui::Node]
#[derive(Debug, Clone, Component)]
#[require(Node)]
pub struct ProgressBar {
    pub foreground: Color,
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
        }
    }

    pub fn with_color(mut self, foreground: Color) -> Self {
        self.foreground = foreground;
        self
    }

    pub fn set_value(&mut self, value: f32) {
        if let Ok(mut store) = self.value.lock() {
            *store = value;
        }
    }

    pub fn set_range(&mut self, min: f32, max: f32) {
        if let Ok(mut store) = self.min.lock() {
            *store = min;
        }
        if let Ok(mut store) = self.max.lock() {
            *store = max;
        }
    }
}

#[derive(Component)]
#[require(
    Node(|| Node {
        height: Val::Percent(100.0),
        ..default()     
    })
)]
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
        (*value - *min) / (*max - *min)
    }
}

fn create_progress_bars(
    trigger: Trigger<OnAdd, ProgressBar>,
    mut commands: Commands,
    mut query: Query<&ProgressBar>,
) {
    if let Ok(data) = query.get_mut(trigger.entity()) {
        // add foreground
        commands.entity(trigger.entity()).with_children(|parent| {
            // foreground
            parent.spawn((
                Node {
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(data.foreground),
                ProgressBarForeground::new(data),
            ));
        });
    }
}

fn update_progress_bars(mut child_query: Query<(&ProgressBarForeground, &mut Node)>) {
    for (data, mut node) in child_query.iter_mut() {
        node.width = Val::Percent(100.0 * data.percent());
    }
}
