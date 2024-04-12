use bevy::prelude::*;
use super::components::*;
use super::styles::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
) {
    commands.entity(main_menu_query.single()).despawn_recursive();
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands.spawn(
        (
            NodeBundle {
                style: WRAPP_STYLE,
                background_color: MENU_BG_COLOR.into(),
                ..default()
            },
            MainMenu {}
        )
    )
    .with_children(|parent| {
        //--- Title ---
        parent.spawn(
            NodeBundle {
                style: TITLE_STYLE,
                background_color: BLOCK_COLOR.into(),
                ..default()
            }
        ).with_children(|parent| {
            //--- Image 1 ---
            parent.spawn(
                ImageBundle {
                    style: IMAGE_STYLE,
                    image: asset_server.load("sprites/ball_blue_large.png").into(),
                    ..default()
                }
            );
            //--- Text ---
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "LASER",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 50.0,
                                    color: Color::RED
                                }
                            ),
                            TextSection::new(
                                "\nBall Game",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
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
            //--- Image 2 ---
            parent.spawn(
                ImageBundle {
                    style: IMAGE_STYLE,
                    image: asset_server.load("sprites/ball_red_large.png").into(),
                    ..default()
                }
            );
        });
        //--- Play button ---
        parent.spawn(
            (
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: BLOCK_COLOR.into(),
                    ..default()
                },
                PlayButton {}
            )
        )
        .with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "PLAY",
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
        //--- Quit button ---
        parent.spawn(
            (
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: BLOCK_COLOR.into(),
                    ..default()
                },
                QuitButton {}
            )
        )
        .with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "QUIT",
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
    })
    .id();

    main_menu_entity
}