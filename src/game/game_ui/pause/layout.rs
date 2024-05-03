use bevy::prelude::*;
use super::components::*;
use super::styles::*;

pub fn spawn_pause_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    hud_query: Query<Entity, With<PauseMenu>>,
) {
    commands.entity(hud_query.single()).despawn_recursive();
}

pub fn build_pause_menu(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
) -> Entity {
    let main_menu_entity = commands.spawn(
        (
            NodeBundle {
                style: WRAPP_STYLE,
                background_color: MENU_BG_COLOR.into(),
                ..default()
            },
            PauseMenu {}
        )
    )
    .with_children(|parent| {
        parent.spawn(
            NodeBundle {
                style: MENU,
                background_color: MENU_BG_COLOR.into(),
                ..default()
            },
        ).with_children(|parent|{
            //--- Title ---
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "GAME PAUSED",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 50.0,
                                    color: Color::RED
                                }
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
            );
            //--- Resume button ---
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: BUTTON_COLOR.into(),
                        ..default()
                    },
                    ResumeButton {}
                )
            )
            .with_children(|parent| {
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "RESUME",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 32.0,
                                        color: Color::WHITE
                                    }
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });
            //--- ToMainMenu button ---
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: BUTTON_COLOR.into(),
                        ..default()
                    },
                    ToMainMenuButton {}
                )
            )
            .with_children(|parent| {
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "TO MAIN MENU",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 32.0,
                                        color: Color::WHITE
                                    }
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });
        });
    })
    .id();

    main_menu_entity
}