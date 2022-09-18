use std::{collections::HashSet, time::Duration};
use rand::seq::IteratorRandom;

use bevy::{prelude::*, time::FixedTimestep};
use bevy_ecs_ldtk::{LdtkEntity, prelude::RegisterLdtkObjects};
use bevy_inspector_egui::Inspectable;
use iyes_loopless::prelude::*;

use crate::{statemanagement::{GameState, PauseState}, level::GroundDetection};

pub struct BugPlugin;

impl Plugin for BugPlugin {
    fn build(&self, app: &mut App) {
        // info!("BugPlugin");
        app
            .insert_resource(BugSpawnTimer(Timer::new(Duration::from_secs(2), true)))
            .register_ldtk_entity::<BugSpawnerBundle>("BugSpawner")
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::GamePlaying)
                    .run_not_in_state(PauseState::Paused)
                    .with_system(spawn_bugs)
                    .into(),
            )
            // .add_system_set(
            //     SystemSet::new()
            //         .with_run_criteria(FixedTimestep::step(1.0))
            //         .with_system(move_bugs)
            //         .into()
            // )
            ;
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

#[derive(Component, Default, Debug)]
pub struct Bug;

#[derive(Bundle, Default, LdtkEntity)]
pub struct BugBundle {
    bug: Bug,

    climber: Climber,

    ground_detection: GroundDetection,

    #[bundle]
    #[sprite_sheet_bundle("graphics/sprites/bug_sprite_001.png", 16.0, 16.0, 5, 1, 0.0, 0.0, 0)]
    sprite_sheet: SpriteSheetBundle,
}

/* System to spawn bugs */
pub fn spawn_bugs(mut commands: Commands,
                  mut bug_spawner_query: Query<&mut Transform, With<BugSpawner>>,
                  bugs_query: Query<&mut Transform, With<Bug>>,
                  time: Res<Time>,
                  mut timer: ResMut<BugSpawnTimer>,
                  ){
    // info!("spawn bugs");

    if bugs_query.iter().count() > 5 {
        return;
    }

    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();

        let mut bug_spawners: Vec<&Transform> = bug_spawner_query.iter().choose_multiple(&mut rng, 1);
        // info!("timer finished");
        for (mut bug_spawner) in bug_spawners.iter_mut() {
            // info!("Bug spawner:");// {}", bug_spawner.entity_instance);
            commands.spawn_bundle(BugBundle::default());
        }
    }
}

// pub fn move_bugs(mut bug_query: Query<(&mut Transform), With<Bug>>){
//     for (mut bug_transform) in bug_query.iter_mut() {
//         // info!("Move bugs");

//         // bug_transform.translation.y += 100.0;
//     }
// }

