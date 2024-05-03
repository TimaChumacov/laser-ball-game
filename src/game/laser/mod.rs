use bevy::prelude::*;
use crate::AppState;
use crate::GameState;

mod systems;
use systems::*;

pub mod components;
use components::*;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LaserSpawnTimer>()
        .add_system(despawn_lasers.in_schedule(OnExit(AppState::Game)))
        .add_systems(
            (
                tick_laser_spawn_timer,
                spawn_lasers_over_time,
                move_lasers,
                laser_animation,
                laser_collision
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(GameState::Running))
        );
    }
}