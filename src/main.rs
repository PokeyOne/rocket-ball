use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f64,
    y: f64
}

#[derive(Component)]
struct Velocity {
    x: f64,
    y: f64
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
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
