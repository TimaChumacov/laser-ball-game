use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use crate::Enemy;
pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LaserSpawnTimer>()
        .add_system(tick_laser_spawn_timer)
        .add_system(spawn_lasers_over_time)
        .add_system(move_lasers);
    }
}

pub const LASER_SPAWN_TIME: f32 = 3.0;

#[derive (Component)]
pub struct Laser {
    pub pivot_a: Option<Entity>,
    pub pivot_b: Option<Entity>,
}

#[derive(Resource)]
pub struct LaserSpawnTimer {
    pub timer: Timer
}

impl Default for LaserSpawnTimer {
    fn default() -> Self {
        LaserSpawnTimer { timer: Timer::from_seconds(LASER_SPAWN_TIME, TimerMode::Repeating) }
    }
}

pub fn tick_laser_spawn_timer(mut laser_spawn_timer: ResMut<LaserSpawnTimer>, time: Res<Time>) {
    laser_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_lasers_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut laser_spawn_timer: ResMut<LaserSpawnTimer>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if laser_spawn_timer.timer.finished() {
        let mut pivot_a = None;
        let mut pivot_b = None;
        for transform in enemy_query.iter() {
            pivot_a = Some(transform);
            pivot_b = Some(transform);
        }
        

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                texture: asset_server.load("sprites/laser.png"),
                ..default()
            },
            Laser {
                pivot_a: pivot_a,
                pivot_b: pivot_b,
            },
        ));
    }
}

pub fn move_lasers( mut lasers_query: Query<(&mut Transform, &Laser)>) {
    for (mut laser_transform, laser) in lasers_query.iter_mut() {
        laser_transform.translation.x += 1.0;
    }
}