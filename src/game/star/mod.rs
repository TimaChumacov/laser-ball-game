use bevy::prelude::*;

use crate::AppState;
use crate::GameState;

mod systems;
use systems::*;

pub mod components;
use components::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
        .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
        .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)))
        .add_systems(
            (
                tick_star_spawn_timer,
                spawn_stars_over_time,
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(GameState::Running))
        );
    }
}