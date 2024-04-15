use bevy::prelude::*;
use crate::AppState;

pub mod styles;
pub mod components;

mod hud_layout;
use hud_layout::*;

mod systems;
use systems::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
        .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
        //.add_system(update_hp);
        // .add_systems(
        //     (
        //         update_hp,
        //     )
        //     .in_set(OnUpdate(AppState::Game))
        // );
    }
}