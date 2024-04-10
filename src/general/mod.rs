use bevy::prelude::*;
use crate::AppState;
use super::GameState;

mod systems;
use systems::*;

pub mod components;

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
        .add_system(toggle_pause.run_if(in_state(AppState::Game)))
        .add_system(enter_game_state)
        .add_system(enter_main_menu)
           .add_systems(
                (
                    entity_collision,
                    confine_entities_in_screen,
                )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameState::Running))
            );
    }
}