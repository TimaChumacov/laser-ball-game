use bevy::prelude::*;

pub const ENEMY_COUNT: i32 = 4;
pub const ENEMY_SPAWN_TIME: f32 = 800.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;

#[derive(Component)]
pub struct Enemy {
    pub id: i32,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer { timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating) }
    }
}
