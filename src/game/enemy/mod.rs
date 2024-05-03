use bevy::prelude::*;
use crate::AppState;
use super::GameState;

mod systems;
use systems::*;

pub mod components;
use components::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
           .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
           .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)))
           .add_systems(
                (
                    enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameState::Running))
            );
    }
}