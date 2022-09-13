use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use iyes_loopless::prelude::*;

use crate::{
    assets::AudioAssets,
    statemanagement::{GameState, PauseState},
};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        debug!("Setting up MusicPlugin");
        app.add_enter_system(
            GameState::GamePlaying,
            start_gameplay_background_music,
        )
        .add_exit_system(GameState::GamePlaying, stop_gameplay_background_music)
        .add_enter_system(PauseState::Paused, pause_background_music)
        .add_exit_system(PauseState::Paused, unpause_background_music);
    }
}

fn start_gameplay_background_music(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    debug!("Playing music");
    audio.play(audio_assets.soothing_nature.clone()).looped();
    audio.set_volume(0.4);
}

fn stop_gameplay_background_music(audio: Res<Audio>) {
    audio.stop();
}

fn pause_background_music(audio: Res<Audio>) {
    audio.pause();
}

fn unpause_background_music(audio: Res<Audio>) {
    audio.resume();
}
