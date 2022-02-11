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

struct WindowSizeState {
    width: f32,
    height: f32
}

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

fn gravity_system(mut query: Query<(&Gravity, &mut Velocity)>) {
    for (_, mut v) in query.iter_mut() {
        v.value += Vec3::new(0.0, -0.2, 0.0);
    }
}

fn velocity_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut t, v) in query.iter_mut() {
        t.translation += v.value;
    }
}

fn wall_collision_system(mut query: Query<(&mut Transform, &mut Velocity)>, win_state: Res<WindowSizeState>) {
    let bottom_border = -(win_state.height/2.0 - BALL_SIZE);

    for (mut t, mut v) in query.iter_mut() {
        let vertical_velocity = v.value.to_array()[1];
        if t.translation.to_array()[1] < bottom_border && vertical_velocity < 0.0 {
            v.value += Vec3::new(0.0, -1.90 * vertical_velocity, 0.0);
            t.translation.y = bottom_border;
        }
    }
}

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
