use bevy::render::camera::ScalingMode;
use bevy::window::Window;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

mod conway_game_of_life_;

const COLLUMS: i32 = 20;
const ROWS: i32 = 20;

#[derive(Component)]
enum GameState {
    Play,
    Pause,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(input)
        .add_system(draw_game)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let game = conway_game_of_life_::Game::new(ROWS as usize, COLLUMS as usize, vec![]);
    commands.spawn(game);
    commands.spawn(GameState::Pause);
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::None,
            ..default()
        },
        ..default()
    });
    // Rectangle
}

fn draw_grid(board: &Vec<Vec<i32>>, mut commands: Commands) {
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.65 * board[y][x] as f32, 0.25, 0.75),

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

fn draw_game(game_query: Query<&conway_game_of_life_::Game>, mut commands: Commands) {
    let game = game_query.single();

    draw_grid(&game.board, commands);
}

fn input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut windows: ResMut<Windows>,
    mut game_query: Query<&mut conway_game_of_life_::Game>,
    mut game_state_query: Query<&mut GameState>,
) {
    let mut game = game_query.single_mut();
    let mut game_state = game_state_query.single();
    if keyboard_input.just_pressed(KeyCode::Space) {
        game_state = match game_state {
            GameState::Pause => &GameState::Play,
            GameState::Play => &GameState::Pause,
        }
    }
    match game_state {
        GameState::Pause => {
            if buttons.just_pressed(MouseButton::Left) {
                let mut width = 0.0;
                let mut height = 0.0;
                for window in windows.iter_mut() {
                    let cursor_position = window.cursor_position().unwrap();
                    width = window.width();
                    height = window.height();
                    let y_index = get_index(height, ROWS, cursor_position[1]);
                    let x_index = get_index(width, COLLUMS, cursor_position[0]);
                    game.set_alive(y_index as usize, x_index as usize);
                    game.set_alive(y_index as usize + 1, x_index as usize + 1);
                    game.set_alive(y_index as usize + 2, x_index as usize + 2);
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.71, 0.40, 0.75),

                            custom_size: Some(Vec2::new(1.6 / ROWS as f32, 1.6 / ROWS as f32)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            (1.8 * x_index as f32 / COLLUMS as f32) - 0.8,
                            (1.8 * y_index as f32 / ROWS as f32) - 0.8,
                            0.,
                        )),
                        ..default()
                    });
                }
            }
        }
        GameState::Play => game.update_board(),
    }
}

fn get_index(window_dimension: f32, rows_or_collums: i32, cursor_position: f32) -> i32 {
    let block = window_dimension / rows_or_collums as f32;
    let index = (window_dimension - cursor_position) / block.floor();
    rows_or_collums - index.floor() as i32 - 1
}
