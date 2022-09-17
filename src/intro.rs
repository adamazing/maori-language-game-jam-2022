use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{level::CameraWayPointBundle, statemanagement::GameState};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::GameIntro, begin_intro);
    }
}

fn begin_intro(
    mut commands: Commands,
    waypoints: Query<Entity, With<CameraWayPointBundle>>,
    current_state: Res<CurrentState<GameState>>,
) {
    debug!("begin_intro");
    commands.insert_resource(NextState(GameState::GamePlaying));
}
