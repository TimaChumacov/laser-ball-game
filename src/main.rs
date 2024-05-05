use bevy::prelude::*;

mod game;
use game::{GamePlugin, GameState};

mod main_menu;
use main_menu::MainMenuPlugin;

mod general;
use general::GeneralPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    .add_plugin(GamePlugin)
    .add_plugin(MainMenuPlugin)
    .add_plugin(GeneralPlugin)
    .run()
}

// The 3 core states of the app
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}