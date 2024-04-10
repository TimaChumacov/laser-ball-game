use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::general::components::Velocity;
use crate::game::player::components::PlayerStats;
use super::components::*;

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
                    random::<f32>() * window.height(),
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

pub fn spawn_enemy_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    mut player_stats: ResMut<PlayerStats>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

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
    }
}