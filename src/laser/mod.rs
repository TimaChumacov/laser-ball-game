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
    pub pivot_a_id: i32,
    pub pivot_b_id: i32,
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
    enemy_query: Query<&Enemy>,
) {
    if laser_spawn_timer.timer.finished() {
        let mut pivot_a_id = 0;
        let mut pivot_b_id = 0;
        for enemy in enemy_query.iter() {
            pivot_a_id = enemy.id;
            pivot_b_id = enemy.id + 1;
        }
        

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                texture: asset_server.load("sprites/developerArrow.png"),
                ..default()
            },
            Laser {
                pivot_a_id: pivot_a_id,
                pivot_b_id: pivot_b_id,
            },
        ));
    }
}

pub fn move_lasers( 
    mut lasers_query: Query<(&mut Transform, &Laser)>,
    enemy_query: Query<(&Transform, &Enemy), Without<Laser>>,
) {
    for (mut laser_transform, laser) in lasers_query.iter_mut() {
        let mut pivot_a: Transform = Transform::from_xyz(0.0, 0.0, 0.0);
        let mut pivot_b: Transform = Transform::from_xyz(0.0, 0.0, 0.0);
        for (enemy_transform, enemy) in enemy_query.iter() {
            if enemy.id == laser.pivot_a_id {
                pivot_a = *enemy_transform;
            }
            if enemy.id == laser.pivot_b_id {
                pivot_b = *enemy_transform;
            }
        }
        laser_transform.translation = pivot_a.translation;//(pivot_a.translation + pivot_b.translation) / 2.0;
        laser_transform.scale.x = 0.2;
        //println!("angle is {}", pivot_a.translation.angle_between(pivot_b.translation));
        //laser_transform.rotation = Quat::from_rotation_z(((pivot_a.translation + pivot_b.translation) / 2.0).angle_between(pivot_b.translation));
        //laser_transform.rotation = laser_transform.looking_at(pivot_b.translation, Vec3::X).rotation;
        laser_transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, pivot_b.translation.angle_between(pivot_a.translation));
    }
}