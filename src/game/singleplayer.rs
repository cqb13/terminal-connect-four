use super::{GameMode, GameResult, GameState};

pub fn play_singleplayer() -> GameResult {
    let mut game_state = GameState::new(GameMode::SinglePlayer);

    game_state.drop_piece_in_column(1).expect("Fail drop 1");
    game_state.switch_player();
    game_state.drop_piece_in_column(2).expect("Fail drop 2");
    game_state.switch_player();
    game_state.drop_piece_in_column(2).expect("Fail drop 3");
    let winner = game_state.are_four_connected();
    println!("Winner: {:?}", winner);
    game_state.drop_piece_in_column(3).expect("Fail drop 3");
    game_state.drop_piece_in_column(4).expect("Fail drop 3");
    game_state.drop_piece_in_column(5).expect("Fail drop 3");
    game_state.switch_player();
    game_state.drop_piece_in_column(6).expect("Fail drop 3");
    game_state.drop_piece_in_column(7).expect("Fail drop 3");
    game_state.drop_piece_in_column(2).expect("Fail drop 2");
    game_state.drop_piece_in_column(2).expect("Fail drop 2");
    game_state.drop_piece_in_column(2).expect("Fail drop 2");
    game_state.switch_player();
    game_state.drop_piece_in_column(2).expect("Fail drop 2");
    game_state.drop_piece_in_column(4).expect("Fail drop 2");
    game_state.drop_piece_in_column(5).expect("Fail drop 2");
    game_state.drop_piece_in_column(5).expect("Fail drop 2");
    game_state.switch_player();
    game_state.drop_piece_in_column(6).expect("Fail drop 2");
    game_state.drop_piece_in_column(6).expect("Fail drop 2");
    game_state.switch_player();
    game_state.drop_piece_in_column(6).expect("Fail drop 2");
    let winner = game_state.are_four_connected();
    game_state.draw_board();
    println!("Winner: {:?}", winner);

    GameResult::new()
}
