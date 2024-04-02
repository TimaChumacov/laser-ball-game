use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::Velocity;

pub const ENEMY_COUNT: i32 = 3;
pub const ENEMY_SPAWN_time: f32 = 3.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
           .add_startup_system(spawn_enemies)
           .add_system(enemy_movement)
           .add_system(tick_enemy_spawn_timer)
           .add_system(spawn_enemy_over_time);
    }
}

#[derive(Component)]
pub struct Enemy {}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer { timer: Timer::from_seconds(ENEMY_SPAWN_time, TimerMode::Repeating) }
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
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
            Enemy {},
        ));
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
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
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
            Enemy {},
        ));
    }
}