use bevy::prelude::*;
use super::components::*;
use super::styles::*;

pub fn spawn_hud(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    build_hud(&mut commands, &asset_server);
}

pub fn despawn_hud(
    mut commands: Commands,
    hud_query: Query<Entity, With<HUD>>,
) {
    commands.entity(hud_query.single()).despawn_recursive();
}

pub fn build_hud(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
) -> Entity {
    let main_menu_entity = commands.spawn(
        (
            NodeBundle {
                style: WRAPP_STYLE,
                ..default()
            },
            HUD {}
        )
    )
    .with_children(|parent| {
        parent.spawn(
            NodeBundle {
                style: HUD_BLOCK_STYLE,
                background_color: HUD_BLOCK_COLOR.into(),
                ..default()
            }
        )
        .with_children(|parent| {
            //--- Hp section ---
            parent.spawn(
                NodeBundle {
                    style: HUD_BLOCK_HALVES_STYLE,
                    ..default()
                },
            ).with_children(|parent| {
                parent.spawn(
                ImageBundle {
                    style: HUD_IMAGE_STYLE,
                    image: asset_server.load("sprites/ball_blue_large.png").into(),
                    ..default()
                }
                );
                parent.spawn((
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "3",
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
                    HitpointsText {}
                ));
            });
            //--- Score section ---
            parent.spawn(
                NodeBundle {
                    style: HUD_BLOCK_HALVES_STYLE,
                    ..default()
                },
            ).with_children(|parent| {
                parent.spawn(
                ImageBundle {
                    style: HUD_IMAGE_STYLE,
                    image: asset_server.load("sprites/star.png").into(),
                    ..default()
                }
                );
                parent.spawn((
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "0",
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
                    ScoreText {}
                ));
            });
        });
        parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "[ESC] to pause",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 12.0,
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
    })
    .id();

    main_menu_entity
}