use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use bevy_inspector_egui::widgets::{InspectorQuery, InspectorQuerySingle};
use heron::prelude::*;
use iyes_loopless::prelude::ConditionSet;
use leafwing_input_manager::prelude::*;

use crate::level::GroundDetection;
use crate::statemanagement::{GameState, PauseState};

pub struct KiwiPlugin;

impl Plugin for KiwiPlugin {
    fn build(&self, app: &mut App) {
        debug!("Setting up KiwiPlugin");
        app.add_plugin(InputManagerPlugin::<KiwiAction>::default())
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::GamePlaying)
                    .run_not_in_state(PauseState::Paused)
                    .with_system(movement)
                    .with_system(animate_kiwi)
                    .into(),
            )
            .add_plugin(InspectorPlugin::<InspectorQuerySingle<Entity, With<Kiwi>>>::new())
            .register_ldtk_entity::<KiwiBundle>("Kiwi");
    }
}

#[derive(Inspectable)]
struct Inspector {
    root_elements: InspectorQuery<With<Kiwi>>
}

#[derive(Component, Default, Debug, Inspectable)]
pub struct Kiwi;

#[derive(Bundle, Default, LdtkEntity)]
struct KiwiBundle {
    kiwi: Kiwi,

    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,

    #[bundle]
    #[from_entity_instance]
    collider_bundle: ColliderBundle,

    #[bundle]
    input_manager: KiwiInput,

    pub ground_detection: GroundDetection,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum KiwiAction {
    Up,
    Down,
    Left,
    Right,
    Peck,
}

#[derive(Bundle)]
pub struct KiwiInput {
    #[bundle]
    input_manager: InputManagerBundle<KiwiAction>,
}

impl Default for KiwiInput {
    fn default() -> Self {
        use KiwiAction::*;

        Self {
            input_manager: InputManagerBundle::<KiwiAction> {
                input_map: InputMap::new([
                    (KeyCode::A, Left),
                    (KeyCode::D, Right),
                    (KeyCode::Left, Left),
                    (KeyCode::Right, Right),
                    (KeyCode::Space, Peck),
                ]),
                ..default()
            },
        }
    }
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: CollisionShape,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: RotationConstraints,
    pub physic_material: PhysicMaterial,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        let rotation_constraints = RotationConstraints::lock();

        match entity_instance.identifier.as_ref() {
            "Kiwi" => ColliderBundle {
                collider: CollisionShape::Sphere { radius: 8. },
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        let rotation_constraints = RotationConstraints::lock();

        info!("{:?}", int_grid_cell);
        if int_grid_cell.value == 1 || int_grid_cell.value == 3 {
            ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(8., 8., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Sensor,
                rotation_constraints,
                ..Default::default()
            }
        } else {
            ColliderBundle::default()
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, true))
    }
}

fn animate_kiwi(
    mut kiwi_query: Query<
        (
            &mut Velocity,
            &mut TextureAtlasSprite,
        ),
        With<Kiwi>
    >,
    mut animation_timer: Local<AnimationTimer>,
    time: Res<Time>
) {

    for (mut velocity, mut sprite) in kiwi_query.iter_mut() {
        sprite.index = if velocity.linear.x > 0. {
            // info!("animate right");
           sprite.flip_x = false;
           animation_timer.0.tick(time.delta());
           let index = 6 + ((sprite.index + if animation_timer.0.finished(){
               animation_timer.0.reset();
               1
           } else {
               0
           }) % 4);
           info!(" Sprite index: {}", index);
           index
        } else if velocity.linear.x < 0. {
           // info!("animate left");
           sprite.flip_x = true;
           animation_timer.0.tick(time.delta());
           let index = 6 + ((sprite.index + if animation_timer.0.finished(){
               animation_timer.0.reset();
               1
           } else {
               0
           }) % 4);
           info!(" Sprite index: {}", index);
           index
        } else {
            // sprite.index = 4;
            // info!("animate idle if time has passed");
            6
        }
        // info!("{}", velocity.linear.x);
    }
}


fn movement(
    mut query: Query<
        (&mut Velocity, &mut ActionState<KiwiAction>),
        (With<Kiwi>, Changed<ActionState<KiwiAction>>),
    >
) {
    // debug!("Movement");
    for (mut velocity, action_state) in query.iter_mut() {
        // debug!("In query loop");
        let right = if action_state.pressed(KiwiAction::Right) {
            1.
        } else {
            0.
        };
        let left = if action_state.pressed(KiwiAction::Left) {
            1.
        } else {
            0.
        };

        velocity.linear.x = (right - left) * 100.;

        if velocity.linear.x > 0. {
            debug!("heading right?");
        } else if velocity.linear.x < 0. {
            debug!("heading left?");
        } else {
            debug!("stationary");
        }
        // info!("{} {}", left, right);

        if action_state.pressed(KiwiAction::Peck) {
            info!("Pecking");
            // velocity.linear.y = 250.;
            // climber.climbing = false;
        }
    }
}
