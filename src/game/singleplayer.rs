use super::{GameResult, GameState, GameMode};

pub fn play_singleplayer() -> GameResult {
    let mut game_state = GameState::new(GameMode::SinglePlayer);

    game_state.drop_piece_in_column(1).expect("Fail drop 1");
    game_state.switch_player();
    game_state.drop_piece_in_column(2).expect("Fail drop 2");
    game_state.switch_player();
    game_state.drop_piece_in_column(2).expect("Fail drop 3");
    game_state.draw_board();

    GameResult::new()
}
