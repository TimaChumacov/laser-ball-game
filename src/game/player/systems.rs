use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::general::components::Velocity;
use crate::game::star::components::Star;
use crate::game::enemy::components::Enemy;
use crate::general::components::ENTITY_SPRITE_DIAMETER;
use crate::AppState;
use super::components::*;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Player)>,
    time: Res<Time>,
) {
    for (mut transform, mut player_velocity, player) in player_query.iter_mut() {
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
        transform.translation += player_velocity.acceleration * PLAYER_SPEED * time.delta_seconds();
        player_velocity.decelerate(&player);
    }

}

pub fn player_attacked(
    enemy_query: Query<&Transform, With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    mut player_stats: ResMut<PlayerStats>,
) {
    let player_trans = player_query.single();
    for enemy_trans in enemy_query.iter() {
        if !player_stats.invincible && player_trans.translation.distance(enemy_trans.translation) < ENTITY_SPRITE_DIAMETER {
            player_stats.hitpoints -= 1;
            player_stats.invincible = true;
            println!("player got hit by an enemy. hp remaining: {}", player_stats.hitpoints);
        }
    }
}

pub fn tick_invincibility_timer(mut invincibility_timer: ResMut<InvincibilityTimer>, time: Res<Time>, mut player_stats: ResMut<PlayerStats>) {
    if player_stats.invincible == true {
        invincibility_timer.timer.tick(time.delta());
        println!("time spent invincible: {}", invincibility_timer.timer.elapsed_secs());
        if invincibility_timer.timer.finished() {
            player_stats.invincible = false;
            invincibility_timer.timer.reset();
        }
    }
}

pub fn kill_player(
    player_stats: ResMut<PlayerStats>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if player_stats.hitpoints <= 0 {
        println!("player is dead");
        next_app_state.set(AppState::GameOver);
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
            acc_mod: 0.02,
            dec_mod: 0.01,
            acc_max: 2.0,
        },
    ));
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
) {
    commands.entity(player_query.single()).despawn();
}

impl Default for PlayerStats {
    fn default() -> Self {
        PlayerStats {
            score: 0,
            hitpoints: 555,
            enemy_count: 0,
            invincible: false,
        }
    }
}

pub fn collect_stars(
    mut commands: Commands,
    star_query: Query<(Entity, &Transform),  With<Star>>,
    mut player_query: Query<&Transform, With<Player>>,
    mut player_stats: ResMut<PlayerStats>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for player_trans in player_query.iter_mut() {
        for (star_entity, star_trans) in star_query.iter() {
            // the value of 48.0 is eyeballed because I've changed star sprite a lot
            if player_trans.translation.distance(star_trans.translation) < 48.0 {
                player_stats.score += 1;
                audio.play(asset_server.load("audio/star.ogg"));
                println!("Star collected! Score: {}", player_stats.score);
                commands.entity(star_entity).despawn();
            }
        }
    }
}