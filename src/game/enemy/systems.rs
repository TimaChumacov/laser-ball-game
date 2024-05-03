use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use std::time::Duration;
use crate::game::laser::components::LaserSpawnTimer;
use crate::general::components::Velocity;
use crate::game::player::components::{Player, PlayerStats};
use super::components::*;

// spawns initial cluster of enemies
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut player_stats: ResMut<PlayerStats>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..ENEMY_COUNT {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    random::<f32>() * window.width(),
                    250.0, // y is fixed to prevent enemies from spawning in player
                    0.0,
                ),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Velocity {
                acceleration: Vec3::new(random::<f32>(), random::<f32>(), 0.0).normalize(),
            },
            Enemy {
                id: player_stats.enemy_count,
            },
        ));
        player_stats.enemy_count += 1;
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn enemy_movement(
    mut enemies_query: Query<(&mut Transform, &mut Velocity), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut enm_transform, enm_velocity) in enemies_query.iter_mut() {
        enm_transform.translation += enm_velocity.acceleration * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    mut player_stats: ResMut<PlayerStats>,
    mut laser_timer: ResMut<LaserSpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let mut random_x: f32;
        let mut random_y: f32;
        // prevents enemies frm spawning too close to the player
        loop { 
            random_x = random::<f32>() * window.width();
            random_y = random::<f32>() * window.height();
            let player_transform = player_query.single();
            if Vec3::new(random_x, random_y, 0.0).distance(player_transform.translation) > 150.0 {
                break;
            }
        }
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Velocity {
                acceleration: Vec3::new(random::<f32>(), random::<f32>(), 0.0).normalize(),
            },
            Enemy {
                id: player_stats.enemy_count,
            },
        ));
        player_stats.enemy_count += 1;
        // each enemy also makes the laser shoot more often
        if laser_timer.timer.duration().as_secs_f64() > 1.0 { 
            let reduced_laser_spawnrate = laser_timer.timer.duration().as_secs_f64() - 0.2;
            laser_timer.timer.set_duration(Duration::from_secs_f64(reduced_laser_spawnrate));
        }
    }
}