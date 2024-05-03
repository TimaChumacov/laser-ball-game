use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Player {
    pub acc_mod: f32, // how fast the ball accelerates
    pub dec_mod: f32, // how fast the ball deccelerates
    pub acc_max: f32, // maximum acceleration
}

#[derive(Resource)]
pub struct PlayerStats {
    pub score: i32,
    pub hitpoints: i32,
    pub enemy_count: i32,
    pub invincible: bool, // after being hit the player becomes invincible for a short time
}

#[derive(Resource)]
pub struct InvincibilityTimer {
    pub timer: Timer
}

impl Default for InvincibilityTimer {
    fn default() -> Self {
        InvincibilityTimer { timer: Timer::from_seconds(1.0, TimerMode::Once) }
    }
}