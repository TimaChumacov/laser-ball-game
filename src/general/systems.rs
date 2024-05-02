use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::enemy::components::ENEMY_SIZE;
use crate::game::player::components::PLAYER_SIZE;
use super::components::*;
use crate::game::GameState;

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            ),
            ..default()
        });
}

pub fn confine_entities_in_screen(
    mut entity_query: Query<(&mut Transform, &mut Velocity)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (mut transform, mut velocity) in entity_query.iter_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE / 2.0;
        let x_min: f32 = 0.0 + half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_min: f32 = 0.0 + half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        if((transform.translation.x < x_min ||
            transform.translation.x > x_max) &&
            velocity.acceleration.x.abs() > 0.7) ||
          ((transform.translation.y < y_min ||
            transform.translation.y > y_max) &&
            velocity.acceleration.y.abs() > 0.7) {
            println!("{}", velocity.acceleration.length());
            //velocity.acceleration -= 1.0;
            audio.play(asset_server.load("audio/bump.ogg"));
        }

        if transform.translation.x < x_min {
            transform.translation.x = x_min;
            velocity.reverse_x();
        } else if transform.translation.x > x_max {
            transform.translation.x = x_max;
            velocity.reverse_x();
        }

        if transform.translation.y < y_min {
            transform.translation.y = y_min;
            velocity.reverse_y();
        } else if transform.translation.y > y_max {
            transform.translation.y = y_max;
            velocity.reverse_y();
        }
    }
}

pub fn entity_collision(
    mut entity_query: Query<(&mut Transform, &mut Velocity)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let mut combinations = entity_query.iter_combinations_mut();
    while let Some([(mut transform_1, mut velocity_1), (mut transform_2, mut velocity_2)]) = combinations.fetch_next() {
        if transform_1.translation.distance(transform_2.translation) < ENEMY_SIZE {
            let mut dir_away: Vec3;
            
            dir_away = (transform_1.translation - transform_2.translation).normalize();
            dir_away *= (ENEMY_SIZE - transform_1.translation.distance(transform_2.translation)) / 2.0;

            transform_1.translation += dir_away;
            transform_2.translation -= dir_away;
            
            let new_vel_1 = velocity_2.acceleration;
            let new_vel_2 = velocity_1.acceleration;

            if new_vel_1 != Vec3::ZERO {
            velocity_1.acceleration = new_vel_1.normalize();
            }
            if new_vel_2 != Vec3::ZERO {
            velocity_2.acceleration = new_vel_2.normalize();
            }

            audio.play(asset_server.load("audio/bump.ogg"));
        }
    }
}

pub fn toggle_pause(
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if game_state.0 == GameState::Running {
            next_game_state.set(GameState::Paused);
            println!("paused.");
        }
        if game_state.0 == GameState::Paused {
            next_game_state.set(GameState::Running);
            println!("running.");
        }
    }
}

// pub fn enter_game_state(
//     mut commands: Commands,
//     keyboard_input: Res<Input<KeyCode>>,
//     app_state: Res<State<AppState>>,
//     mut next_app_state: ResMut<NextState<AppState>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::L) {
//         if app_state.0 == AppState::MainMenu {
//             next_app_state.set(AppState::Game);
//             commands.insert_resource(PlayerStats::default());
//             println!("in game.");
//         }
//     }
// }

// pub fn enter_main_menu(
//     keyboard_input: Res<Input<KeyCode>>,
//     app_state: Res<State<AppState>>,
//     game_state: Res<State<GameState>>,
//     mut next_app_state: ResMut<NextState<AppState>>,
//     mut next_game_state: ResMut<NextState<GameState>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Escape) {
//         if app_state.0 == AppState::Game {
//             next_app_state.set(AppState::MainMenu);
//             println!("in main menu.");
//         }
//         if game_state.0 == GameState::Paused {
//             next_game_state.set(GameState::Running);
//         }
//     }
// }