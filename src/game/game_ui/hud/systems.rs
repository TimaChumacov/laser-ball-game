use bevy::prelude::*;
use crate::game::player::components::PlayerStats;
use super::components::{HitpointsText, ScoreText};

pub fn update_hud(
    mut hitpoints_query: Query<&mut Text, With<HitpointsText>>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<HitpointsText>)>,
    player_stats: Res<PlayerStats>,
) {
    let mut hitpoints_text = hitpoints_query.single_mut();
    let mut score_text = score_query.single_mut();
    if player_stats.is_changed() {
        hitpoints_text.sections[0].value = format!("{}", player_stats.hitpoints.to_string());
        score_text.sections[0].value = format!("{}", player_stats.score.to_string());
    }
}
