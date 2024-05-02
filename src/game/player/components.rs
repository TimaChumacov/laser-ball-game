use bevy::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 10.0;

#[derive(Component)]
pub struct Player {
    pub acc_mod: f32,
    pub dec_mod: f32,
    pub acc_max: f32,
}

#[derive(Resource)]
pub struct PlayerStats {
    pub score: i32,
    pub hitpoints: i32,
    pub enemy_count: i32,
    pub invincible: bool,
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