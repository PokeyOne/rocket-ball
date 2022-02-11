//! A small test-project for the Bevy game-engine.
//!
//! Essentially there is a ball on the screen that is affected by gravity, and
//! it will bounce off the walls. The user may also press buttons to activate
//! thrusters side-to-side and up-and-down. That's it.

use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy::window::WindowResized;
use bevy::app::Events;

/// 60 fps time step
const TIME_STEP: f64 = 1.0 / 60.0;
/// Size of the ball used for collision and such.
const BALL_SIZE: f32 = 15.0;

/// Entities that implement this will be affected by gravity if they also
/// implement the [`Velocity`] component.
#[derive(Component)]
struct Gravity;

/// Used for entities that track their speed. Such as the ball.
///
/// See also [`Gravity`]
#[derive(Component)]
struct Velocity {
    value: Vec3
}

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity {
            value: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

/// A resouce used to resize things based on the current size of the window.
struct WindowSizeState {
    width: f32,
    height: f32
}

/// Setup all the entities such as the cameras and the ball
fn setup(mut commands: Commands) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Ball
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Gravity)
        .insert(Velocity::default());
}

/// Apply the acceleration of gravity to entities that implement velocity,
/// and gravity
fn gravity_system(mut query: Query<(&Gravity, &mut Velocity)>) {
    for (_, mut v) in query.iter_mut() {
        v.value += Vec3::new(0.0, -0.2, 0.0);
    }
}

/// Apply transformation due to velocity to all entities with position
/// and velocity
fn velocity_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut t, v) in query.iter_mut() {
        t.translation += v.value;
    }
}

/// Handle collisions witht the sides of the window
fn wall_collision_system(mut query: Query<(&mut Transform, &mut Velocity)>, win_state: Res<WindowSizeState>) {
    // The location of the bottom border
    let bottom_border = -(win_state.height/2.0 - BALL_SIZE);

    for (mut t, mut v) in query.iter_mut() {
        let vertical_velocity = v.value.to_array()[1];
        if t.translation.to_array()[1] < bottom_border && vertical_velocity < 0.0 {
            v.value += Vec3::new(0.0, -1.90 * vertical_velocity, 0.0);
            t.translation.y = bottom_border;
        }
    }
}

/// This event shall be triggered every time the window size updates, and it
/// will update the resource [`WindowSizeState`] with the new size.
///
/// The [`WindowSizeState`] is used by collision and various things for dynamic
/// positioning and collision calculations based on the window size.
fn resize_detector(resize_event: Res<Events<WindowResized>>, mut win_state: ResMut<WindowSizeState>) {
    let mut reader = resize_event.get_reader();
    for event in reader.iter(&resize_event) {
        win_state.width = event.width;
        win_state.height = event.height;
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rocket Ball".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(WindowSizeState { width: 800.0, height: 600.0 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(gravity_system)
                .with_system(velocity_system)
                .with_system(wall_collision_system)
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(resize_detector)
        .run();
}
