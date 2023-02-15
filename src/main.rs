use bevy::prelude::*;
use bevy::window::WindowMode;

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        mode: WindowMode::BorderlessFullscreen,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.7, 0.3),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(96.0, 96.0, 10.0),
                ..default()
            },
            ..default()
        }
    );
}

fn transforming(mut query: &mut Query<&mut Transform>) {
    for mut transform in query.iter_mut() {
        transform.translation.x += 1.0;
    }
}