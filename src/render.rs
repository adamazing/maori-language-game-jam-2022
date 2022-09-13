use bevy::{prelude::*, render::texture::ImageSettings};
use iyes_loopless::prelude::AppLooplessStateExt;
use leafwing_input_manager::Actionlike;

use crate::{assets::BackgroundLayerAssets, statemanagement::GameState};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App)  {
        app
            .insert_resource(ImageSettings::default_nearest())
            .add_startup_system(initialize_camera_system)
            .add_enter_system(GameState::GamePlaying, spawn_background_layers);
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum CameraControlTemp {
    Up,
    Down,
    Left,
    Right
}

pub fn spawn_background_layers(mut commands: Commands, background_layers: Res<BackgroundLayerAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0,0.0,0.0),
            texture: background_layers.background_layer_1.clone(),
            ..default()
        });
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0,0.0,1.0),
            texture: background_layers.background_layer_2.clone(),
            ..default()
        });
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0,0.0,2.0),
            texture: background_layers.background_layer_3.clone(),
            ..default()
        });
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0,0.0,3.0),
            texture: background_layers.background_layer_4.clone(),
            ..default()
        })
        ;
}

pub fn initialize_camera_system(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_xyz(0.0,0.0,40.0),
           ..default()
        });
}
