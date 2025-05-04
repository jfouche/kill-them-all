use bevy::prelude::*;

///
/// Define the [ProgressBar] color
///
#[derive(Component, Reflect, Deref)]
pub struct ProgressBarColor(pub Color);

impl Default for ProgressBarColor {
    fn default() -> Self {
        ProgressBarColor(Color::WHITE)
    }
}

/// The [ProgressBar] component should be nested with a [bevy::ui::Node]
///
/// The background color is defined with the [BackgroundColor] component, and
/// the foreground color is defined with the [ProgressBarColor] component.
#[derive(Component, Default, Debug, Clone, Reflect)]
#[require(Node, ProgressBarColor)]
pub struct ProgressBar {
    pub min: f32,
    pub max: f32,
    pub value: f32,
}

impl ProgressBar {
    fn percent(&self) -> f32 {
        (self.value - self.min) / (self.max - self.min)
    }
}

#[derive(Component)]
#[require(
    Node {
        height: Val::Percent(100.0),
        ..default()
    }
)]
struct ProgressBarForeground;

///
///  A [Plugin] to mange [ProgressBar]s
///
pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ProgressBarColor>()
            .register_type::<ProgressBar>()
            .add_systems(Update, update_progress_bars)
            .add_observer(create_progress_bar);
    }
}

fn create_progress_bar(trigger: Trigger<OnAdd, ProgressBar>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(children![ProgressBarForeground]);
}

fn update_progress_bars(
    mut children: Query<(&mut Node, &mut BackgroundColor, &ChildOf), With<ProgressBarForeground>>,
    parents: Query<(&ProgressBar, &ProgressBarColor)>,
) {
    for (mut node, mut background, child_of) in children.iter_mut() {
        if let Ok((data, color)) = parents.get(child_of.parent()) {
            node.width = Val::Percent(100.0 * data.percent());
            *background = BackgroundColor(**color);
        }
    }
}
