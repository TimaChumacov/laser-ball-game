use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::Velocity;
use crate::Star;
use crate::enemy::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Player_stats>()
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_system(player_attacked)
        .add_system(collect_stars);
    }
}

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 10.0;

#[derive(Component)]
pub struct Player {
    pub acc_mod: f32,
    pub dec_mod: f32,
    pub acc_max: f32,
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
        transform.translation += player_velocity.acceleration * PLAYER_SPEED * time.delta_seconds();
        player_velocity.decelerate(&player);
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

#[derive(Resource)]
pub struct Player_stats {
    pub score: i32,
    pub hitpoints: i32,
    pub enemy_count: i32,
}


impl Default for Player_stats {
    fn default() -> Self {
        Player_stats {
            score: 0,
            hitpoints: 3,
            enemy_count: 0,
        }
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