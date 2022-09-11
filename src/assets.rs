use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::statemanagement::GameState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        info!("Loading assets");
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<AudioAssets>()
                .with_collection::<FontAssets>()
                .continue_to_state(GameState::GamePlaying),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/Baloo-Regular.ttf")]
    pub baloo: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/music/rest-and-recover-by-chilledmusic.ogg")]
    pub rest_and_recover: Handle<AudioSource>,

    #[asset(path = "audio/music/soothing-nature-by-chilledmusic.ogg")]
    pub soothing_nature: Handle<AudioSource>,
}

