use bevy::prelude::*;

mod hud;
use hud::HUDPlugin;

mod pause;
use pause::PausePlugin;

mod gameover;
use gameover::GameOverPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HUDPlugin)
           .add_plugin(PausePlugin)
           .add_plugin(GameOverPlugin);
    }
}