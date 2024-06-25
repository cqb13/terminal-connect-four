use super::{GameMode, GameResult, GameState};

pub fn play_singleplayer() -> GameResult {
    let mut game_state = GameState::new(GameMode::SinglePlayer);

    GameResult::new(0, Some(super::Player::PlayerOne))
}
