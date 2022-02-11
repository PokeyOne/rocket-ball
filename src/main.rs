use bevy::prelude::*;
use bevy::core::FixedTimestep;

/// 60 fps time step
const TIME_STEP: f64 = 1.0 / 60.0;

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        v.value += Vec3::new(0.0, -0.1, 0.0);
    }
}

fn velocity_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut t, v) in query.iter_mut() {
        t.translation += v.value;
    }
}

fn wall_collision_system(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut t, mut v) in query.iter_mut() {
        let vertical_velocity = v.value.to_array()[1];
        if t.translation.to_array()[1] < -300.0 && vertical_velocity < 0.0 {
            v.value += Vec3::new(0.0, -1.90 * vertical_velocity, 0.0);
        }
    }
}

fn main() {
    App::new()
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
        .run();
}
