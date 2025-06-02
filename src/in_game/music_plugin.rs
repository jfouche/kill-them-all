use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource, audio::music, components::despawn_all, schedule::GameState,
};

pub fn music_plugin(app: &mut App) {
    app.register_type::<InGameMusicAsset>()
        .load_resource::<InGameMusicAsset>()
        .add_systems(OnEnter(GameState::InGame), start_music)
        .add_systems(OnExit(GameState::InGame), despawn_all::<InGameMusic>);
}

#[derive(Component)]
struct InGameMusic;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct InGameMusicAsset {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for InGameMusicAsset {
    fn from_world(world: &mut World) -> Self {
        error!("FromWorld for InGameMusicAsset");
        Self {
            music: world.load_asset("audio/Goblins_Den_Regular.ogg"),
        }
    }
}

fn start_music(mut commands: Commands, asset: Res<InGameMusicAsset>) {
    commands.spawn((
        Name::new("InGameMusic"),
        InGameMusic,
        music(asset.music.clone()),
    ));
}
