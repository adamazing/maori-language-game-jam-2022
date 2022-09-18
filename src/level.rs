use std::collections::{HashMap, HashSet};

use bevy::{
    prelude::*,
    render::camera::{DepthCalculation, ScalingMode, WindowOrigin},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
// use bevy_inspector_egui::Inspectable;
use heron::prelude::*;
use iyes_loopless::prelude::*;

// use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};

use crate::{
    assets::LevelAsset,
    kiwi::Kiwi,
    statemanagement::{GameState, PauseState},
};

pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::Uid(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
                set_clear_color: SetClearColor::No,
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            // .add_plugin(NoCameraPlayerPlugin)
            .add_startup_system(setup_camera)
            .add_enter_system(GameState::GameIntro, spawn_level)
            .register_ldtk_int_cell::<ForestFloorBundle>(1)
            .register_ldtk_int_cell::<BoardBundle>(2)
            .register_ldtk_int_cell::<TreeBundle>(3)
            .register_ldtk_entity::<CameraWayPointBundle>("CameraWayPoint")
            .add_system(pause_physics_during_load)
            .add_system(spawn_wall_collision)
            .add_system(spawn_ground_sensor)
            .add_system(ground_detection)
            .add_enter_system(PauseState::Paused, pause_physics)
            .add_exit_system(PauseState::Paused, unpause_physics)
            .add_system(restart_level);
    }
}

#[derive(Component, Default, Debug, Inspectable)]
pub struct CameraWayPoint;

#[derive(Bundle, Component, Default)]
pub struct CameraWayPointBundle {
    waypoint: CameraWayPoint,
}

impl LdtkEntity for CameraWayPointBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlas>,
    ) -> CameraWayPointBundle {
        println!("CameraWayPointBundle added, here are some facts:");
        for field_instance in &entity_instance.field_instances {
            println!(
                "    its {} {}",
                field_instance.identifier,
                explain_field(&field_instance.value)
            );
        }

        CameraWayPointBundle { ..default() }
    }
}

fn explain_field(value: &FieldValue) -> String {
    match value {
        FieldValue::Int(Some(i)) => format!("has an integer of {}", i),
        FieldValue::Float(Some(f)) => format!("has a float of {}", f),
        FieldValue::Bool(b) => format!("is {}", b),
        FieldValue::String(Some(s)) => format!("says {}", s),
        FieldValue::Color(c) => format!("has the color {:?}", c),
        FieldValue::Enum(Some(e)) => format!("is the variant {}", e),
        FieldValue::FilePath(Some(f)) => format!("references {}", f),
        FieldValue::Point(Some(p)) => format!("is at ({}, {})", p.x, p.y),
        a => format!("is hard to explain: {:?}", a),
    }
}

fn setup_camera(mut commands: Commands) {
    debug!("Spawning camera");
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            depth_calculation: DepthCalculation::ZDifference,
            scaling_mode: ScalingMode::None,
            window_origin: WindowOrigin::Center,
            scale: 0.43,
            left: -640.0, right: 640.0,
            bottom: -420.0, top: 420.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0,0.0,40.0),
        ..default()
    })
    // .insert(FlyCam)
        ;
}

fn normalise_camera_within_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        (Without<Kiwi>, With<Camera2d>),
    >,
    player_query: Query<&Transform, With<Kiwi>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Kiwi>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    windows: Res<Windows>,
) {
    debug!("Normalise camera in level");
    let window = windows.primary();
    let aspect_ratio: f32 = window.width() / window.height();

    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) =
            camera_query.single_mut();

        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    let level_ratio =
                        level.px_wid as f32 / ldtk_level.level.px_hei as f32;

                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::None;
                    orthographic_projection.bottom = 0.;
                    orthographic_projection.left = 0.;
                    if level_ratio > aspect_ratio {
                        orthographic_projection.top =
                            (level.px_hei as f32 / 9.).round() * 9.;
                        orthographic_projection.right =
                            orthographic_projection.top * aspect_ratio;
                        camera_transform.translation.x = (player_translation.x
                            - level_transform.translation.x
                            - orthographic_projection.right / 2.)
                            .clamp(
                                0.,
                                level.px_wid as f32
                                    - orthographic_projection.right,
                            );
                        camera_transform.translation.y = 0.;
                    } else {
                        orthographic_projection.right =
                            (level.px_wid as f32 / 16.).round() * 16.;
                        orthographic_projection.top =
                            orthographic_projection.right / aspect_ratio;
                        camera_transform.translation.y = (player_translation.y
                            - level_transform.translation.y
                            - orthographic_projection.top / 2.)
                            .clamp(
                                0.,
                                level.px_hei as f32
                                    - orthographic_projection.top,
                            );
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x +=
                        level_transform.translation.x;
                    camera_transform.translation.y +=
                        level_transform.translation.y;
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct InvisibleWallBundle {
    wall: Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climbable;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ForestFloorBundle {
    ground: Wall,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct BoardBundle {
    ground: Wall,
    climbable: Climbable,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct TreeBundle {
    wall: Wall,
    climbable: Climbable,
}


fn spawn_level(mut commands: Commands, level: Res<LevelAsset>) {
    debug!("Spawning level");

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: level.scene.clone(),
        transform: Transform::from_xyz(-660., -300., 5.),
        ..default()
    });
}

/// Spawns heron collisions for the walls of a level
///
/// You could just insert a ColliderBundle in to the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// The algorithm used here is a nice compromise between simplicity, speed,
/// and a small number of rectangle colliders.
/// In basic terms, it will:
/// 1. consider where the walls are
/// 2. combine wall tiles into flat "plates" in each individual row
/// 3. combine the plates into rectangles across multiple rows wherever possible
/// 4. spawn colliders for each rectangle
pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> =
        HashMap::new();

    wall_query.for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_insert_with(HashSet::new)
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_handle)| {
            if let Some(level_walls) =
                level_to_wall_locations.get(&level_entity)
            {
                let level = levels
                    .get(level_handle)
                    .expect("Level should be loaded by this point");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level
                    .level
                    .layer_instances
                    .clone()
                    .expect("Level asset should have layers")[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right
                    // edge
                    for x in 0..width + 1 {
                        match (
                            plate_start,
                            level_walls.contains(&GridCoords { x, y }),
                        ) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut wall_rects: Vec<Rect> = Vec::new();
                let mut previous_rects: HashMap<Plate, Rect> = HashMap::new();

                // an extra empty row so the algorithm "terminates" the rects that touch the top
                // edge
                plate_stack.push(Vec::new());

                for (y, row) in plate_stack.iter().enumerate() {
                    let mut current_rects: HashMap<Plate, Rect> =
                        HashMap::new();
                    for plate in row {
                        if let Some(previous_rect) =
                            previous_rects.remove(plate)
                        {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    top: previous_rect.top + 1,
                                    ..previous_rect
                                },
                            );
                        } else {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    bottom: y as i32,
                                    top: y as i32,
                                    left: plate.left,
                                    right: plate.right,
                                },
                            );
                        }
                    }

                    // Any plates that weren't removed above have terminated
                    wall_rects.append(
                        &mut previous_rects.values().copied().collect(),
                    );
                    previous_rects = current_rects;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn()
                            .insert(CollisionShape::Cuboid {
                                half_extends: Vec3::new(
                                    (wall_rect.right as f32
                                        - wall_rect.left as f32
                                        + 1.)
                                        * grid_size as f32
                                        / 2.,
                                    (wall_rect.top as f32
                                        - wall_rect.bottom as f32
                                        + 1.)
                                        * grid_size as f32
                                        / 2.,
                                    0.,
                                ),
                                border_radius: None,
                            })
                            .insert(RigidBody::Static)
                            .insert(PhysicMaterial {
                                friction: 0.1,
                                ..Default::default()
                            })
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32
                                    * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}

pub fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<
        (Entity, &CollisionShape, &Transform),
        Added<GroundDetection>,
    >,
) {
    for (entity, shape, transform) in detect_ground_for.iter() {
        if let CollisionShape::Cuboid { half_extends, .. } = shape {
            let detector_shape = CollisionShape::Cuboid {
                half_extends: Vec3::new(half_extends.x / 2., 2., 0.),
                border_radius: None,
            };

            let sensor_translation =
                Vec3::new(0., -half_extends.y, 0.) / transform.scale;

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn()
                    .insert(RigidBody::Sensor)
                    .insert(detector_shape)
                    .insert(Transform::from_translation(sensor_translation))
                    .insert(GlobalTransform::default())
                    .insert(GroundSensor {
                        ground_detection_entity: entity,
                        intersecting_ground_entities: HashSet::new(),
                    });
            });
        } else if let CollisionShape::Sphere { radius } = shape {
            info!("{:?}", radius);

            let detector_shape = CollisionShape::Cuboid {
                half_extends: Vec3::new(radius / 2., 2., 0.),
                border_radius: None,
            };

            let sensor_translation =
                Vec3::new(0., -radius, 0.) / transform.scale;

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn()
                    .insert(RigidBody::Sensor)
                    .insert(detector_shape)
                    .insert(Transform::from_translation(sensor_translation))
                    .insert(GlobalTransform::default())
                    .insert(GroundSensor {
                        ground_detection_entity: entity,
                        intersecting_ground_entities: HashSet::new(),
                    });
            });
        }
    }
}

pub fn ground_detection(
    mut ground_detectors: Query<&mut GroundDetection>,
    mut ground_sensors: Query<(Entity, &mut GroundSensor)>,
    mut collisions: EventReader<CollisionEvent>,
    rigid_bodies: Query<&RigidBody>,
) {
    for (entity, mut ground_sensor) in ground_sensors.iter_mut() {
        for collision in collisions.iter() {
            match collision {
                CollisionEvent::Started(a, b) => {
                    match rigid_bodies.get(b.rigid_body_entity()) {
                        Ok(RigidBody::Sensor) => {
                            // don't consider sensors to be "the ground"
                        }
                        Ok(_) => {
                            if a.rigid_body_entity() == entity {
                                ground_sensor
                                    .intersecting_ground_entities
                                    .insert(b.rigid_body_entity());
                            }
                        }
                        Err(_) => {
                            panic!("If there's a collision, there should be an entity")
                        }
                    }
                }
                CollisionEvent::Stopped(a, b) => {
                    if a.rigid_body_entity() == entity {
                        ground_sensor
                            .intersecting_ground_entities
                            .remove(&b.rigid_body_entity());
                    }
                }
            }
        }

        if let Ok(mut ground_detection) =
            ground_detectors.get_mut(ground_sensor.ground_detection_entity)
        {
            ground_detection.on_ground =
                !ground_sensor.intersecting_ground_entities.is_empty();
        }
    }
}

#[derive(Clone, Debug, Default, Component, Inspectable)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}

fn pause_physics_during_load(
    mut level_events: EventReader<LevelEvent>,
    mut physics_time: ResMut<PhysicsTime>,
) {
    for event in level_events.iter() {
        match event {
            LevelEvent::SpawnTriggered(_) => physics_time.set_scale(0.),
            LevelEvent::Transformed(_) => physics_time.set_scale(1.),
            _ => (),
        }
    }
}

fn pause_physics(mut physics_time: ResMut<PhysicsTime>) {
    physics_time.set_scale(0.);
}

fn unpause_physics(mut physics_time: ResMut<PhysicsTime>) {
    physics_time.set_scale(1.);
}

fn restart_level(
    mut commands: Commands,
    level_query: Query<Entity, With<Handle<LdtkLevel>>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::R) {
        for level_entity in level_query.iter() {
            commands.entity(level_entity).insert(Respawn);
        }
    }
}
