use bevy::{prelude::*, transform};
use bevy::window::PrimaryWindow;

pub mod enemy;
use enemy::*;

pub mod star;
use star::*;

pub mod player;
use player::*;

pub mod laser;
use laser::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(EnemyPlugin)
    .add_plugin(StarPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(LaserPlugin)
    .add_startup_system(spawn_camera)
    .add_system(entity_collision)
    .add_system(confine_entities_in_screen)
    .run()
}

impl Velocity {
    pub fn reverse_x(&mut self) {
        self.acceleration.x *= -1.0;
    }

    pub fn reverse_y(&mut self) {
        self.acceleration.y *= -1.0;
    }

    pub fn decelerate(&mut self, player: &Mut<'_, Player>) {
        if self.acceleration.x > 0.0 {
            self.acceleration.x -= player.dec_mod;
        } else if self.acceleration.x < 0.0 {
            self.acceleration.x += player.dec_mod;
        }
        if self.acceleration.y > 0.0 {
            self.acceleration.y -= player.dec_mod;
        } else if self.acceleration.y < 0.0 {
            self.acceleration.y += player.dec_mod;
        }
        //self.acceleration.x *= 0.95;
        //self.acceleration.y *= 0.95;
    }

    pub fn limit_acceleration(&mut self, player: &Mut<'_, Player>) {
        if self.acceleration.x > player.acc_max {
            self.acceleration.x = player.acc_max;
        } else if self.acceleration.x < -player.acc_max {
            self.acceleration.x = -player.acc_max;
        }
        if self.acceleration.y > player.acc_max {
            self.acceleration.y = player.acc_max;
        } else if self.acceleration.y < -player.acc_max {
            self.acceleration.y = -player.acc_max;
        }
    }
}

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
) {
    for (mut transform, mut velocity) in entity_query.iter_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE / 2.0;
        let x_min: f32 = 0.0 + half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_min: f32 = 0.0 + half_player_size;
        let y_max: f32 = window.height() - half_player_size;

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
            
            let new_vel_1: Vec3;
            let new_vel_2: Vec3;
            
            new_vel_1 = velocity_1.acceleration - (velocity_1.acceleration  - velocity_2.acceleration);
            new_vel_2 = velocity_2.acceleration - (velocity_2.acceleration  - velocity_1.acceleration);

            velocity_1.acceleration = new_vel_1.normalize();
            velocity_2.acceleration = new_vel_2.normalize();

            //audio.play(asset_server.load("audio/click.ogg"));
        }
    }
}

#[derive(Component)]
pub struct Velocity {
    acceleration: Vec3,
}