use bevy::prelude::*;

use crate::game::player::components::Player;

#[derive(Component)]
pub struct Velocity {
    pub acceleration: Vec3,
}

impl Velocity {
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