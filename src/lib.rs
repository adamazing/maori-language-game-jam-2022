#![allow(clippy::type_complexity,clippy::too_many_arguments)]
pub use bevy_asset_loader::prelude::*;
pub use iyes_loopless::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

pub const LAUNCHER_TITLE: &str = "Tane Mahuta";

mod music;
mod statemanagement;
mod assets;
mod paused;
mod helpers;

use assets::AssetPlugin;
use music::MusicPlugin;
use paused::PausePlugin;
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
    .add_startup_system(load_icon);
    app
}

fn load_icon(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("bevy.png"),
        ..default()
    });
}