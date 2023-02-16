use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::window::WindowMode;

#[derive(Component)]
struct Protagonist;

#[derive(Component)]
struct Road;

#[derive(Component)]
struct House;

#[derive(Component)]
struct Dustbin;

#[derive(Component)]
struct Beggar;

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        mode: WindowMode::BorderlessFullscreen,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_startup_system(add_road)
    .add_startup_system(add_houses)
    .add_startup_system(add_dustbins)
    .add_startup_system(add_beggar)
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.04))
            .with_system(transforming)
    )
    .run();
}

fn setup(mut commands: Commands, windows: Res<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let window = windows.get_primary().unwrap();
    commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.7, 0.3),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(96.0, 96.0, 10.0),
                translation: Vec3::new(-700.0, 100.0 - window.width()/4.0, 10.0),
                ..default()
            },
            ..default()
        }
    )
    .insert(Protagonist);
}

fn transforming(mut query: Query<&mut Transform, With<Protagonist>>) {
    for mut transform in query.iter_mut() {
        if transform.translation.x < 0.0 {
            transform.translation.x += 1.5;
            return;
        }
        if transform.translation.y < 0.0 {
            transform.translation.y += 0.5;
        }
    }
}

// fn resize_notificator(resize_event: Res<Events<WindowResized>>) {
//     let mut reader = resize_event.get_reader();
//     for e in reader.iter(&resize_event) {
//         println!("width = {} height = {}", e.width, e.height);
//     }
// }
fn add_road(windows: Res<Windows>, mut commands: Commands) {
    let window = windows.get_primary().unwrap();
    println!("Window size was: {},{}", window.width(), window.height());
    commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform { 
                scale: Vec3::new(window.width() * 2.0, 100.0, 10.0),
                translation: Vec3::new(0.0, -window.height()/4.0, 0.0),
                ..default()
            },
            ..default()
        }
    )
    .insert(Road);
}

fn add_houses(mut commands: Commands) {
    let positions = vec![-500.0, 0.0, 500.0];
    for i in positions {
        commands.spawn_bundle(
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(300.0, 300.0, 1.0),
                    translation: Vec3::new(i, 100.0, 0.0),
                    ..default()
                },
                ..default()
            }
        )
        .insert(House);
    }
}

fn add_dustbins(mut commands: Commands) {
    let positions: Vec<_> = vec![-600.0, -100.0, 400.0];
    for i in positions {
        commands.spawn_bundle(
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 1.0),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(48.0, 96.0, 48.0),
                    translation: Vec3::new(i, -6.0, 3.0),
                    ..default()
                },
                ..default()
            }
        )
        .insert(Dustbin);
    }
}

fn add_beggar(mut commands: Commands) {
    commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 1.0, 0.0, 0.2),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(96.0, 96.0, 10.0),
                translation: Vec3::new(-115.0, -8.0, 4.0),
                ..default()
            },
            ..default()
        }
    )
    .insert(Beggar);
}