use bevy::prelude::*;
use crate::AppState;

mod components;
mod styles;

mod layout;
use layout::*;

mod interactions;
use interactions::*;

pub struct MenuUIPlugin;

impl Plugin for MenuUIPlugin {
    fn build(&self, app: &mut App) {
     app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
        .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
        .add_systems(
            (
                interact_with_play_button,
                interact_with_quit_button,
            )
            .in_set(OnUpdate(AppState::MainMenu))
        );
    }
}