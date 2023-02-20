use bevy::core::{FixedTimestep};
use bevy::prelude::*;
use bevy::window::{WindowMode};
use display_info::DisplayInfo;

struct Window {
    width: f32,
    height: f32
}

trait States {}

struct MainScreen;
struct Scene1;

impl States for MainScreen {}

impl States for Scene1 {}

struct GameState<T: States> {
    current: T,
    previous: T
}

//scene 1 spritelist
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

#[derive(Component)]
struct SceneText;

fn main() {

    let display_info = DisplayInfo::all().unwrap()[0];

    App::new()
    .insert_resource(WindowDescriptor {
        mode: WindowMode::BorderlessFullscreen,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .insert_resource(Window {
        width: display_info.width as f32, 
        height: display_info.height as f32
    })
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
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.04))
            .with_system(beggar_movements)
    )
    .run();
}

//sets up only the protagonists and the camera maybe even the ui bundle
fn setup(mut commands: Commands, asset: Res<AssetServer>, window: Res<Window>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "Good day for meals", 
            TextStyle { 
                font_size: 30.0, 
                color: Color::rgb(1.0, 1.0, 1.0), 
                font: asset.load("Pixeled.ttf")
            }, 
            TextAlignment { 
                vertical: VerticalAlign::Center, 
                horizontal: HorizontalAlign::Center 
            }
        ),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 11.0),
            ..default()
        },
        ..default()
    })
    .insert(SceneText);

    let window_width = window.width;
    let window_height = window.height;
    commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.7, 0.3),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(96.0, 96.0, 10.0),
                translation: Vec3::new(-window_width / 2.0, 100.0 - window_height / 4.0, 10.0),
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

fn add_road(mut commands: Commands, window: Res<Window>) {
    let window_width = window.width;
    let window_height = window.height;
    println!("Window size was: {},{}", window_width, window_height);
    commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform { 
                scale: Vec3::new(window_width, 100.0, 10.0),
                translation: Vec3::new(0.0, -window_height/4.0, 0.0),
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

fn beggar_movements(
    prota_posi: Query<(&Transform, &Protagonist), Without<Beggar>>, 
    mut beggar_posi: Query<&mut Transform, With<Beggar>>,
    despawner: Query<Entity, With<Beggar>>,
    mut commands: Commands,
    mut text: Query<&mut Text, With<SceneText>>
) {
    for (ppos, _) in prota_posi.iter() {
        if ppos.translation.x > -300.0 {
            for mut bpos in beggar_posi.iter_mut() {
                if bpos.translation.x > -300.0 {
                    text.iter_mut().next().unwrap().sections[0].value = "I better run".to_string();
                    bpos.translation.x -= 2.0;
                    return;
                } 
                if bpos.translation.x <= -300.0 && bpos.translation.y < 100.0 {
                    text.iter_mut().next().unwrap().sections[0].value = "".to_string();
                    bpos.translation.y += 1.0;
                    return;
                }
                
                for entity in despawner.iter() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}