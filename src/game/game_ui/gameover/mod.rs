use bevy::prelude::*;
use crate::AppState;

pub mod styles;
pub mod components;

mod gameover_layout;
use gameover_layout::*;

mod interactions;
use interactions::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_gameover.in_schedule(OnEnter(AppState::GameOver)))
        .add_system(despawn_gameover.in_schedule(OnExit(AppState::GameOver)))
        .add_system(interact_with_to_main_menu_button.in_set(OnUpdate(AppState::GameOver)));
    }
}
