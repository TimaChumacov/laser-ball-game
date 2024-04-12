use bevy::prelude::*;

mod menu_ui;
use menu_ui::MenuUIPlugin;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(main_menu)
        .add_plugin(MenuUIPlugin);
    }
}

pub fn main_menu() {
    println!("Main Menu");
}