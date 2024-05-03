use bevy::prelude::*;
use bevy::window::PrimaryWindow;
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

// works for both the player and the enemies
pub fn confine_entities_in_screen(
    mut entity_query: Query<(&mut Transform, &mut Velocity)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (mut transform, mut velocity) in entity_query.iter_mut() {
        let window = window_query.get_single().unwrap();

        let half_entity_size: f32 = ENTITY_SPRITE_DIAMETER / 2.0;
        let x_min: f32 = 0.0 + half_entity_size;
        let x_max: f32 = window.width() - half_entity_size;
        let y_min: f32 = 0.0 + half_entity_size;
        let y_max: f32 = window.height() - half_entity_size;

        // makes sure staying next to a wall won't trigger a sfx each frame
        if((transform.translation.x < x_min ||
            transform.translation.x > x_max) &&
            velocity.acceleration.x.abs() > 0.7) ||
          ((transform.translation.y < y_min ||
            transform.translation.y > y_max) &&
            velocity.acceleration.y.abs() > 0.7) {
            audio.play(asset_server.load("audio/bump.ogg"));
        }

        // player and enemies get their momentum mirrored after hitting a wall
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

// works for both the player and the enemies
pub fn entity_collision(
    mut entity_query: Query<(&mut Transform, &mut Velocity)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let mut combinations = entity_query.iter_combinations_mut();
    while let Some([(mut transform_1, mut velocity_1), (mut transform_2, mut velocity_2)]) = combinations.fetch_next() {
        if transform_1.translation.distance(transform_2.translation) < ENTITY_SPRITE_DIAMETER {
            let mut dir_away: Vec3;
            
            dir_away = (transform_1.translation - transform_2.translation).normalize();
            dir_away *= (ENTITY_SPRITE_DIAMETER - transform_1.translation.distance(transform_2.translation)) / 2.0;

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