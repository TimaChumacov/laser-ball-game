use bevy::prelude::*;
use crate::AppState;
use crate::game::GameState;
use super::components::*;
use super::styles::{HOVERED_BUTTON_COLOR, BLOCK_COLOR};

pub fn interact_with_to_main_menu_button(
    mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<ToMainMenuButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>
) {
    if let Ok((interaction, mut backgroundcolor)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                next_app_state.set(AppState::MainMenu);
                next_game_state.set(GameState::Running);
            },
            Interaction::Hovered => {
                *backgroundcolor = HOVERED_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *backgroundcolor = BLOCK_COLOR.into();
            }
        }
    }
    
}