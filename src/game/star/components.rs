use bevy::prelude::*;

pub const STAR_COUNT: i32 = 5;
pub const STAR_SPAWN_TIME: f32 = 3.0;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer { timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating) }
    }
}

#[derive (Component)]
pub struct Star {}
