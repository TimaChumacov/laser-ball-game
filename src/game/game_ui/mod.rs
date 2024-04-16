use bevy::prelude::*;

mod hud;
use hud::HUDPlugin;

mod pause;
use pause::PausePlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HUDPlugin)
           .add_plugin(PausePlugin);
        // .add_system(spawn_pause_menu.in_schedule(OnEnter(GameState::Paused)))
        // .add_system(despawn_pause_menu.in_schedule(OnExit(GameState::Paused)))
        // .add_systems(
        //     (
        //         update_hud,
        //     )
        //     .in_set(OnUpdate(AppState::Game))
        // );
    }
}