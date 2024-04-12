use bevy::prelude::*;
use rand::prelude::*;
use crate::game::enemy::components::Enemy;
use crate::game::player::components::Player;
use crate::game::player::components::PlayerStats;

use super::components::*;

pub fn despawn_lasers(
    mut commands: Commands,
    laser_query: Query<Entity, With<Laser>>
) {
    for laser_entity in laser_query.iter() {
        commands.entity(laser_entity).despawn();
    }
}

pub fn tick_laser_spawn_timer(mut laser_spawn_timer: ResMut<LaserSpawnTimer>, time: Res<Time>) {
    laser_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_lasers_over_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    laser_spawn_timer: ResMut<LaserSpawnTimer>,
    player_stats: ResMut<PlayerStats>,
) {
    if laser_spawn_timer.timer.finished() {
        let pivot_a_id = rand::thread_rng().gen_range(0..player_stats.enemy_count - 1);
        let pivot_b_id = pivot_a_id + 1;
        
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                texture: asset_server.load("sprites/seamless_laser.png"),
                ..default()
            },
            Laser {
                pivot_a_id: pivot_a_id,
                pivot_b_id: pivot_b_id,
                lifetime: 0.0,
                damaged_player: false,
            },
        ));
    }
}

pub fn move_lasers(
    mut lasers_query: Query<(&mut Transform, &Laser), Without<Enemy>>,
    enemy_query: Query<(&Transform, &Enemy)>,
) {
    for (mut laser_transform, laser) in lasers_query.iter_mut() {
        let (pivot_a, pivot_b) = find_enemies_by_id(&enemy_query, laser.pivot_a_id, laser.pivot_b_id);
        let dir_to_pivot_b = (pivot_b.translation - pivot_a.translation).normalize();
        let quat_to_pivot_b = Quat::from_rotation_arc(Vec3::X, dir_to_pivot_b);
        laser_transform.rotation = quat_to_pivot_b;
        laser_transform.translation = (pivot_a.translation + pivot_b.translation) / 2.0;
        laser_transform.scale.x = Vec3::distance(pivot_a.translation, pivot_b.translation) / LASER_SPRITE_WIDTH;
    }
}

pub fn find_enemies_by_id(enemy_query: &Query<(&Transform, &Enemy)>, a_id: i32, b_id: i32) -> (Transform, Transform) {
    let mut pivot_a: Transform = Transform::from_xyz(0.0, 0.0, 0.0);
        let mut pivot_b: Transform = Transform::from_xyz(0.0, 0.0, 0.0);
    for (enemy_transform, enemy) in enemy_query.iter() {
        if enemy.id == a_id {
            pivot_a = *enemy_transform;
        }
        if enemy.id == b_id {
            pivot_b = *enemy_transform;
        }
    }
    return (pivot_a, pivot_b);
}

pub fn laser_animation( 
    mut commands: Commands,
    mut lasers_query: Query<(Entity, &mut Transform, &mut Laser)>,
    time: Res<Time>,
) {
    for (laser_entity, mut laser_transform, mut laser) in lasers_query.iter_mut() {
        laser.lifetime += time.delta_seconds();
        match laser.lifetime {
            x if x < 0.3 => laser_transform.scale.y = laser.lifetime * 2.0,
            x if x < 0.6 => laser_transform.scale.y = (laser.lifetime - 0.3) * 2.0,
            x if x < 1.3 => laser_transform.scale.y = 0.0,
            x if x > 2.0 => commands.entity(laser_entity).despawn(),
            _ => laser_transform.scale.y = 1.0,
        }
    }
}

pub fn laser_collision( 
    mut lasers_query: Query<&mut Laser>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(&Transform, &Enemy)>,
    mut player_stats: ResMut<PlayerStats>,
) {
    for mut laser in lasers_query.iter_mut() {
        let (pivot_a, pivot_b) = find_enemies_by_id(&enemy_query, laser.pivot_a_id, laser.pivot_b_id);
        let player_transform = player_query.single();
        let distance_between_pivots = Vec3::distance(pivot_a.translation, pivot_b.translation);
        let player_distance_to_pivots = Vec3::distance(player_transform.translation, pivot_a.translation) + 
                                             Vec3::distance(player_transform.translation, pivot_b.translation);
        if (player_distance_to_pivots - distance_between_pivots) < 5.75 && !laser.damaged_player && laser.lifetime > 1.3 {
            player_stats.hitpoints -= 1;
            println!("laser touched!!! {}", player_stats.hitpoints);
            laser.damaged_player = true;
        }
    }
}