use bevy::prelude::*;
use crate::game::GameState;

pub mod styles;
pub mod components;

mod pause_layout;
use pause_layout::*;

mod interactions;
use interactions::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_pause_menu.in_schedule(OnEnter(GameState::Paused)))
        .add_system(despawn_pause_menu.in_schedule(OnExit(GameState::Paused)))
        .add_systems(
            (
                interact_with_resume_button,
                interact_with_to_main_menu_button,
            )
            .in_set(OnUpdate(GameState::Paused))
        );
    }
}
