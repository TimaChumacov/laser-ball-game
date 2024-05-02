use bevy::app::AppExit;
use bevy::prelude::*;
use crate::AppState;
use super::components::*;
use super::styles::{HOVERED_BUTTON_COLOR, BLOCK_COLOR};
use crate::game::player::components::PlayerStats;

pub fn interact_with_play_button(
    mut commands: Commands,
    mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut backgroundcolor)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                next_app_state.set(AppState::Game);
                commands.insert_resource(PlayerStats::default());
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

pub fn interact_with_quit_button(
    mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut backgroundcolor)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                app_exit_event_writer.send(AppExit);
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