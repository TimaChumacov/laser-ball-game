use bevy::prelude::*;
use super::components::{HitpointsText};

pub fn update_hp(
    hitpoints_query: Query<&mut Text, With<HitpointsText>>,
    asset_server: &Res<AssetServer>,
) {
    let hitpoints_text = hitpoints_query.single();
    hitpoints_text.sections = vec![
            TextSection::new(
                "2",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE
                }
            )
        ]
}