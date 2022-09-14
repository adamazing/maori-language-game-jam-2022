#![allow(clippy::type_complexity, clippy::too_many_arguments)]
use bevy::prelude::*;
pub use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioPlugin;
pub use iyes_loopless::prelude::*;

pub const LAUNCHER_TITLE: &str = "Tane Mahuta";

mod assets;
mod debug;
mod helpers;
mod music;
mod paused;
mod statemanagement;
mod render;

use assets::AssetPlugin;
use debug::DebugPlugin;
use music::MusicPlugin;
use paused::PausePlugin;
use render::RenderPlugin;
use statemanagement::{GameState, PauseState};

pub fn app() -> App {
    let mut app = App::new();
    app.add_loopless_state(GameState::Loading)
        .add_loopless_state(PauseState::UnPaused)
        .insert_resource(WindowDescriptor {
            title: LAUNCHER_TITLE.to_string(),
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetPlugin)
        .add_plugin(MusicPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(PausePlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(RenderPlugin);
    app
}

