use bevy::prelude::*;
use crate::AppState;

pub mod styles;
pub mod components;

mod layout;
use layout::*;

mod systems;
use systems::*;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
        .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)))
        .add_system(update_hud.in_set(OnUpdate(AppState::Game)));
    }
}
