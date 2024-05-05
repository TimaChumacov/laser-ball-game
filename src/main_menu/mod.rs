use bevy::prelude::*;

mod menu_ui;
use menu_ui::MenuUIPlugin;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MenuUIPlugin);
    }
}