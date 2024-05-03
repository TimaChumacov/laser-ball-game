use bevy::prelude::*;
use crate::AppState;
use crate::GameState;

mod systems;
use systems::*;

pub mod components;
use components::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerStats>()
        .init_resource::<InvincibilityTimer>()
        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        .add_system(despawn_player.in_schedule(OnExit(AppState::Game)))
        .add_systems(   
            (
                player_movement,
                player_attacked,
                tick_invincibility_timer,
                kill_player,
                collect_stars
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(GameState::Running))
        );
    }
}