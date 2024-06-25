use super::{GameMode, GameResult, GameState};
use crate::tui::refresh_display;
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};

pub fn play_multiplayer() -> GameResult {
    let mut game_state = GameState::new(GameMode::Multiplayer);
    let possible_moves = game_state.height * game_state.width;

    while game_state.turns < possible_moves {
        let mut current_column = 4;
        loop {
            game_state.draw_board_with_marker(current_column);
            terminal::enable_raw_mode().expect("Failed to enable raw mode");
            let event = read().unwrap();
            match event {
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => match code {
                    KeyCode::Char('q') => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        println!("Quitting...");
                        std::process::exit(0);
                    }
                    KeyCode::Left => {
                        if current_column > 1 {
                            current_column -= 1;
                        } else {
                            current_column = game_state.width;
                        }
                    }
                    KeyCode::Right => {
                        if current_column < game_state.width {
                            current_column += 1;
                        } else {
                            current_column = 1;
                        }
                    }
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        let result = game_state.drop_piece_in_column(current_column);

                        match result {
                            Ok(_) => {
                                refresh_display(15);
                                break;
                            }
                            Err(_) => {}
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
            terminal::disable_raw_mode().expect("Failed to disable raw mode");
            refresh_display(15);
        }

        let winner = game_state.are_four_connected();

        match winner {
            Some(winner) => {
                game_state.draw_board();
                return GameResult::new(game_state.turns, Some(winner));
            }
            None => {}
        }

        game_state.switch_player();
    }

    game_state.draw_board();
    GameResult::new(game_state.turns, None)
}
