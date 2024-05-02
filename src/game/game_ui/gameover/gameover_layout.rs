use bevy::prelude::*;
use super::components::*;
use super::styles::*;
use crate::game::player::components::PlayerStats;

pub fn spawn_gameover(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    player_stats: Res<PlayerStats>,
) {
    let _hud_entity = build_gameover(&mut commands, &asset_server, &player_stats);
}

pub fn despawn_gameover(
    mut commands: Commands,
    hud_query: Query<Entity, With<GameOverScreen>>,
) {
    commands.entity(hud_query.single()).despawn_recursive();
}

pub fn build_gameover(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    player_stats: &Res<PlayerStats>,
) -> Entity {
    let main_menu_entity = commands.spawn(
        (
            NodeBundle {
                style: WRAPP_STYLE,
                background_color: MENU_BG_COLOR.into(),
                ..default()
            },
            GameOverScreen {}
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
                                "GAME OVER",
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
            //--- Score section ---
            parent.spawn(
                NodeBundle {
                    style: BUTTON_STYLE,
                    ..default()
                },
            ).with_children(|parent| {
                parent.spawn(
                ImageBundle {
                    style: IMAGE_STYLE,
                    image: asset_server.load("sprites/star.png").into(),
                    ..default()
                }
                );
                parent.spawn((
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    player_stats.score.to_string(),
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
                    },
                    FinalScore {}
                ));
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