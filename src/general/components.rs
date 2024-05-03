use bevy::prelude::*;
use crate::game::player::components::Player;

// player and enemies are the same size
pub const ENTITY_SPRITE_DIAMETER: f32 = 64.0;

// Velocity is here because both the player and the enemies use it
#[derive(Component)]
pub struct Velocity {
    pub acceleration: Vec3,
}

impl Velocity {
    // reverse are used for mirroring momentum after hitting a wall
    pub fn reverse_x(&mut self) {
        self.acceleration.x *= -1.0;
    }

    pub fn reverse_y(&mut self) {
        self.acceleration.y *= -1.0;
    }

    pub fn decelerate(&mut self, player: &Mut<'_, Player>) {
        if self.acceleration.x > 0.0 {
            self.acceleration.x -= player.dec_mod;
        } else if self.acceleration.x < 0.0 {
            self.acceleration.x += player.dec_mod;
        }
        if self.acceleration.y > 0.0 {
            self.acceleration.y -= player.dec_mod;
        } else if self.acceleration.y < 0.0 {
            self.acceleration.y += player.dec_mod;
        }
    }

    pub fn limit_acceleration(&mut self, player: &Mut<'_, Player>) {
        if self.acceleration.x > player.acc_max {
            self.acceleration.x = player.acc_max;
        } else if self.acceleration.x < -player.acc_max {
            self.acceleration.x = -player.acc_max;
        }
        if self.acceleration.y > player.acc_max {
            self.acceleration.y = player.acc_max;
        } else if self.acceleration.y < -player.acc_max {
            self.acceleration.y = -player.acc_max;
        }
    }
}