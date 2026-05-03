use bevy::prelude::*;
use crate::components::{LivesDisplay, MoneyDisplay, WaveDisplay};
use crate::constants::ENEMIES_PER_WAVE;
use crate::resources::{GameState, GameStatus};


pub fn update_console_ui(game_state: Res<GameState>, game_status: Res<GameStatus>) {
    if *game_status == GameStatus::Playing && game_state.is_changed() {
        println!(
            "Wave: {} | Money: ${} | Lives: {} | Enemies: {}/{}",
            game_state.wave,
            game_state.money,
            game_state.lives,
            game_state.enemy_count,
            game_state.wave * ENEMIES_PER_WAVE
        );
    }
}

pub fn update_game_ui(
    game_state: Res<GameState>,
    game_status: Res<GameStatus>,
    mut ui_query: ParamSet<(
        Query<&mut Text, With<MoneyDisplay>>,
        Query<&mut Text, With<WaveDisplay>>,
        Query<&mut Text, With<LivesDisplay>>,
    )>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }
    for mut text in ui_query.p0().iter_mut() {
        text.sections[0].value = format!("Money: ${}", game_state.money);
    }
    for mut text in ui_query.p1().iter_mut() {
        text.sections[0].value = format!("Wave: {}", game_state.wave);
    }
    for mut text in ui_query.p2().iter_mut() {   
        text.sections[0].value = format!("Lives: {}", game_state.lives);
    }
}