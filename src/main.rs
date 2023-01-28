use bevy::render::camera::ScalingMode;
use bevy::window::Window;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const COLLUMS: i32 = 20;
const ROWS: i32 = 20;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(input)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::None,
            ..default()
        },
        ..default()
    });
    draw_grid(commands);
    // Rectangle
}

fn draw_grid(mut commands: Commands) {
    for y in 0..ROWS {
        for x in 0..COLLUMS {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.65, 0.25, 0.75),

                    custom_size: Some(Vec2::new(1.6 / ROWS as f32, 1.6 / ROWS as f32)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    (1.8 * x as f32 / COLLUMS as f32) - 0.8,
                    (1.8 * y as f32 / ROWS as f32) - 0.8,
                    0.,
                )),
                ..default()
            });
        }
    }
}

fn input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut windows: ResMut<Windows>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let mut width = 0.0;
        let mut height = 0.0;
        for window in windows.iter_mut() {
            let cursor_position = window.cursor_position().unwrap();
            width = window.width();
            height = window.height();
            println!(
                "mouse clicked {:?}",
                get_index(height, ROWS, cursor_position[0])
            );
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.71, 0.40, 0.75),

                    custom_size: Some(Vec2::new(1.6 / ROWS as f32, 1.6 / ROWS as f32)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    (1.8 * get_index(width, COLLUMS, cursor_position[0]) as f32 / COLLUMS as f32)
                        - 0.8,
                    (1.8 * get_index(height, ROWS, cursor_position[1]) as f32 / ROWS as f32) - 0.8,
                    0.,
                )),
                ..default()
            });
        }
    }
}

fn get_index(window_dimension: f32, rows_or_collums: i32, cursor_position: f32) -> i32 {
    let block = window_dimension / rows_or_collums as f32;
    let index = (window_dimension - cursor_position) / block.floor();
    rows_or_collums - index.floor() as i32 - 1
}
