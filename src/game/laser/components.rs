use bevy::prelude::*;

pub const LASER_SPAWN_TIME: f32 = 3.0;
pub const LASER_SPRITE_WIDTH: f32 = 1041.0;

#[derive (Component)]
pub struct Laser {
    // "pivots" are the enemies on which both ends of a laser are. Lasers save the id's of those enemies here.
    pub pivot_a_id: i32,
    pub pivot_b_id: i32,
    pub lifetime: f32,
    pub played_warning_sfx: bool,
    pub played_laser_sfx: bool,
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