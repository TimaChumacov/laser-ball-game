use bevy::prelude::*;

pub mod enemy;
use enemy::EnemyPlugin;

pub mod star;
use star::StarPlugin;

pub mod player;
use player::PlayerPlugin;

pub mod laser;
use laser::LaserPlugin;

pub mod game_ui;
use game_ui::GameUIPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused,
}
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
        .add_plugin(EnemyPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(LaserPlugin)
        .add_plugin(GameUIPlugin);
    }
}

