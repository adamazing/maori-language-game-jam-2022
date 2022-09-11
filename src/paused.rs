use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    assets::FontAssets,
    helpers::despawn_entities_with,
    statemanagement::{GameState, PauseState},
};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PauseMenuAction>::default())
            .add_enter_system(PauseState::Paused, spawn_pause_menu)
            .add_exit_system(
                PauseState::Paused,
                despawn_entities_with::<PauseMenuItem>,
            )
            .add_startup_system(spawn_pause_menu_detector)
            .add_system(exit_game.run_in_state(PauseState::Paused))
            .add_system(
                change_pause_state.run_in_state(GameState::GamePlaying),
            );
    }
}

#[derive(Default, Component)]
struct PauseMenuItem;

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PauseMenuAction {
    Close,
    ExitGame,
    Open,
}

fn spawn_pause_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                margin: UiRect {
                    top: Val::Px(0.0),
                    left: Val::Auto,
                    bottom: Val::Px(0.0),
                    right: Val::Auto,
                },
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: UiRect {
                            top: Val::Px(0.0),
                            left: Val::Auto,
                            bottom: Val::Px(0.0),
                            right: Val::Auto,
                        },
                        ..default()
                    },
                    text: Text::from_section(
                        "Paused".to_string(),
                        TextStyle {
                            font: font_assets.baloo.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                })
                .insert(PauseMenuItem);
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        top: Val::Px(0.0),
                        left: Val::Auto,
                        bottom: Val::Px(0.0),
                        right: Val::Auto,
                    },
                    ..default()
                },
                text: Text::from_section(
                    "Press Enter to continue".to_string(),
                    TextStyle {
                        font: font_assets.baloo.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        top: Val::Px(0.0),
                        left: Val::Auto,
                        bottom: Val::Px(0.0),
                        right: Val::Auto,
                    },
                    ..default()
                },
                text: Text::from_section(
                    "Press Q to exit game".to_string(),
                    TextStyle {
                        font: font_assets.baloo.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        })
        .insert(PauseMenuItem);
}

fn spawn_pause_menu_detector(mut commands: Commands) {
    commands.spawn_bundle(InputManagerBundle {
        input_map: InputMap::new([
            (KeyCode::Escape, PauseMenuAction::Open),
            (KeyCode::Return, PauseMenuAction::Close),
            (KeyCode::Q, PauseMenuAction::ExitGame),
        ]),
        action_state: ActionState::default(),
    });
}

fn exit_game(
    action_query: Query<&ActionState<PauseMenuAction>>,
    current_state: Res<CurrentState<PauseState>>,
) {
    for action in &action_query {
        if action.pressed(PauseMenuAction::ExitGame)
            && matches!(current_state.0, PauseState::Paused)
        {
            std::process::exit(0);
        }
    }
}

fn change_pause_state(
    mut commands: Commands,
    action_query: Query<&ActionState<PauseMenuAction>>,
    current_state: Res<CurrentState<PauseState>>,
) {
    for action in &action_query {
        if action.pressed(PauseMenuAction::Close)
            && matches!(current_state.0, PauseState::Paused)
        {
            commands.insert_resource(NextState(PauseState::UnPaused));
        } else if action.pressed(PauseMenuAction::Open)
            && matches!(current_state.0, PauseState::UnPaused)
        {
            commands.insert_resource(NextState(PauseState::Paused))
        }
    }
}
