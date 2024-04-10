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
        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        .add_system(despawn_player.in_schedule(OnExit(AppState::Game)))
        .add_systems(   
            (
                player_movement,
                //player_attacked,
                collect_stars
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(GameState::Running))
        );
    }
}