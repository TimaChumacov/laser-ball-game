use bevy::{prelude::*, transform};
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub mod enemy;
use enemy::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PlAYER_SPEED: f32 = 10.0;

pub const STAR_COUNT: i32 = 5;
pub const STAR_SPAWN_time: f32 = 3.0;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(EnemyPlugin)
    .init_resource::<Player_stats>()
    .init_resource::<StarSpawnTimer>()
    .add_startup_system(spawn_player)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_stars)
    .add_system(player_movement)
    .add_system(player_attacked)
    .add_system(collect_stars)
    .add_system(entity_collision)
    .add_system(confine_entities_in_screen)
    .add_system(tick_star_spawn_timer)
    .add_system(spawn_stars_over_time)
    .run()
}

#[derive(Component)]
pub struct Player {
    pub acc_mod: f32,
    pub dec_mod: f32,
    pub acc_max: f32,
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

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Player)>,
    time: Res<Time>,
) {
    for (mut transform, mut player_velocity, mut player) in player_query.iter_mut() {
        //let mut direction = Vec3::ZERO;
        let acc_mod = player.acc_mod.clone();
        
        if keyboard_input.pressed(KeyCode::A) {
            player_velocity.acceleration += Vec3::new(-acc_mod, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            player_velocity.acceleration += Vec3::new(acc_mod, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) {
            player_velocity.acceleration += Vec3::new(0.0, acc_mod, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            player_velocity.acceleration += Vec3::new(0.0, -acc_mod, 0.0);
        }
        
        player_velocity.limit_acceleration(&player);
        transform.translation += player_velocity.acceleration * PlAYER_SPEED * time.delta_seconds();
        player_velocity.decelerate(&player);
    }

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
                //transform_2.translation += (velocity_2.acceleration - velocity_1.acceleration).normalize();

                transform_1.translation += dir_away;
                transform_2.translation -= dir_away;
            

            //let normal = (transform_1.translation - transform_2.translation) / (transform_1.translation - transform_2.translation).abs();
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

pub fn player_attacked(
    enemy_query: Query<&Transform, With<Enemy>>,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut player_stats: ResMut<Player_stats>,

) {
    for (player_trans, mut player) in player_query.iter_mut() {
        for enemy_trans in enemy_query.iter() {
            if player_trans.translation.distance(enemy_trans.translation) < ENEMY_SIZE {
                player_stats.hitpoints -= 1;
                println!("{}", player_stats.hitpoints);
            }
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            ),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Velocity {
            acceleration: Vec3::ZERO,
        },
        Player {
            acc_mod: 1.0,
            dec_mod: 0.5,
            acc_max: 100.0,
        },
    ));
}

#[derive(Component)]
pub struct Velocity {
    acceleration: Vec3,
}

#[derive(Resource)]
pub struct Player_stats {
    pub score: i32,
    pub hitpoints: i32,
}


impl Default for Player_stats {
    fn default() -> Self {
        Player_stats {
            score: 0,
            hitpoints: 3,
        }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer { timer: Timer::from_seconds(STAR_SPAWN_time, TimerMode::Repeating) }
    }
}

#[derive (Component)]
pub struct Star {}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..STAR_COUNT {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    random::<f32>() * window.width(),
                    random::<f32>() * window.height(),
                    0.0,
                ),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn collect_stars(
    mut commands: Commands,
    star_query: Query<(Entity, &Transform),  With<Star>>,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut player_stats: ResMut<Player_stats>,
) {
    for (player_trans, mut player) in player_query.iter_mut() {
        for (star_entity, star_trans) in star_query.iter() {
            if player_trans.translation.distance(star_trans.translation) < 48.0 {
                player_stats.score += 1;
                println!("score: {}", player_stats.score);
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}