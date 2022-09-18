use std::{collections::HashSet, time::Duration};
use rand::seq::IteratorRandom;

use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkEntity, prelude::RegisterLdtkObjects};
use bevy_inspector_egui::Inspectable;
use iyes_loopless::prelude::*;

use crate::statemanagement::{GameState, PauseState};

pub struct BugPlugin;

impl Plugin for BugPlugin {
    fn build(&self, app: &mut App) {
        info!("BugPlugin");
        app
            .insert_resource(BugSpawnTimer(Timer::new(Duration::from_secs(2), true)))
            .register_ldtk_entity::<BugSpawnerBundle>("BugSpawner")
            .add_system(
                spawn_bugs
                    .run_in_state(GameState::GamePlaying)
                    .run_not_in_state(PauseState::Paused)
            );
    }
}

pub struct BugSpawnTimer(Timer);

#[derive(Component, Default, Debug, Inspectable)]
pub struct BugSpawner;

#[derive(Bundle, Default, Debug, LdtkEntity)]
pub struct BugSpawnerBundle {
    spawner: BugSpawner,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climber {
    pub climbing: bool,
    pub intersecting_climbables: HashSet<Entity>,
}

/* System to spawn bugs */
pub fn spawn_bugs(mut commands: Commands,
                  mut bug_spawner_query: Query<&mut BugSpawner>,
                  time: Res<Time>,
                  mut timer: ResMut<BugSpawnTimer>,
                  ){
    // info!("spawn bugs");

    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();

        let mut bug_spawners: Vec<&BugSpawner> = bug_spawner_query.iter().choose_multiple(&mut rng, 1);
        info!("timer finished");
        for (mut bug_spawner) in bug_spawners.iter_mut() {
            info!("Bug spawner:");// {}", bug_spawner.entity_instance);
        }
    }
}


