use crate::components::*;
use crate::schedule::*;
use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

#[derive(Component)]
#[require(
    Text(|| Text("Kill'em all".into())),
    TextFont(|| TextFont::from_font_size(80.)),
    TextColor(|| TextColor(Color::WHITE))
)]
struct SplashScreenTitle;

#[derive(Component)]
#[require(
    Text(|| Text("Press any key to continue".into())),
    TextFont(|| TextFont::from_font_size(16.)),
    TextColor(|| TextColor(Color::BLACK))
)]
struct SplashScreenMessage;

#[derive(Component)]
#[component(on_add = create_splash_screen)]
#[require(
    Name(|| Name::new("SplashScreen")),
    Node(|| Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    })
)]
struct SplashScreen;

fn create_splash_screen(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    world.commands().queue(CreateSpashScreen(entity));
}

struct CreateSpashScreen(Entity);

impl Command for CreateSpashScreen {
    fn apply(self, world: &mut World) {
        world.entity_mut(self.0).with_children(|parent| {
            parent.spawn(SplashScreenTitle);
            parent.spawn(SplashScreenMessage);
        });
    }
}
const BACKGROUND_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), spawn_splash_screen)
        .add_systems(OnExit(GameState::Splash), despawn_all::<SplashScreen>)
        .add_systems(Update, goto_main_menu.run_if(in_state(GameState::Splash)));
}

fn spawn_splash_screen(mut commands: Commands) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    commands.spawn(SplashScreen);
}

// fn display_continue(mut messages: Query<&mut Text, With<SplashScreenMessage>>) {
//     for mut text in &mut messages {
//         text.sections[0].value = "Press any key to continue".into();
//     }
// }

fn goto_main_menu(
    mut game_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if keys.get_pressed().len() != 0 {
        game_state.set(GameState::Menu);
    }
    if mouse.pressed(MouseButton::Left) {
        game_state.set(GameState::Menu);
    }
}
