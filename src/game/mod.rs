mod multiplayer;
mod singleplayer;

use self::multiplayer::play_multiplayer;
use self::singleplayer::play_singleplayer;
use crate::display::colors::Color;

const GAME_BOARD_WIDTH: i32 = 7;
const GAME_BOARD_HEIGHT: i32 = 6;

pub enum GameMode {
    SinglePlayer,
    Multiplayer,
}

#[derive(Debug)]
pub enum Player {
    PlayerOne,
    PlayerTwo,
    Computer,
}

impl Player {
    pub fn to_string(&self) -> String {
        match self {
            Player::PlayerOne => "Player One",
            Player::PlayerTwo => "Player Two",
            Player::Computer => "Computer",
        }
        .to_string()
    }

    pub fn color(&self) -> Piece {
        match self {
            Player::PlayerOne => Piece::Red,
            Player::PlayerTwo | Player::Computer => Piece::Yellow,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Red,
    Yellow,
    None,
}

impl Piece {
    pub fn to_string(&self) -> String {
        match self {
            Piece::Red => "R",
            Piece::Yellow => "Y",
            Piece::None => "E",
        }
        .to_string()
    }

    pub fn to_symbol(&self) -> String {
        match self {
            Piece::Red | Piece::Yellow => "⏺",
            Piece::None => " ",
        }
        .to_string()
    }

    pub fn ansi_color(&self) -> String {
        match self {
            Piece::Red => Color::Red,
            Piece::Yellow => Color::Yellow,
            Piece::None => Color::White,
        }
        .to_ansi_color_code()
        .to_string()
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Piece::Red, Piece::Red) => true,
            (Piece::Yellow, Piece::Yellow) => true,
            (Piece::None, Piece::None) => true,
            _ => false,
        }
    }
}

pub struct GameState {
    current_player: Player,
    turns: i32,
    game_mode: GameMode,
    game_board: Vec<Vec<Piece>>,
    width: i32,
    height: i32,
}

impl GameState {
    pub fn new(game_mode: GameMode) -> GameState {
        let game_board =
            vec![vec![Piece::None; GAME_BOARD_WIDTH as usize]; GAME_BOARD_HEIGHT as usize];

        GameState {
            current_player: Player::PlayerOne,
            turns: 0,
            game_mode,
            game_board,
            width: GAME_BOARD_WIDTH,
            height: GAME_BOARD_HEIGHT,
        }
    }

    pub fn drop_piece_in_column(&mut self, column: i32) -> Result<(), String> {
        if column > self.width || column < 1 {
            return Err("Column does not exist on boar".to_string());
        }

        if self.game_board[0][column as usize - 1] != Piece::None {
            return Err("The selected column is full".to_string());
        }

        // finds the first open spot on the column
        for row in 0..self.height {
            if self.game_board[row as usize][column as usize - 1] != Piece::None {
                self.game_board[row as usize - 1][column as usize - 1] =
                    self.current_player.color();
                self.turns += 1;
                return Ok(());
            }
        }

        // the row is empty, place piece at bottom
        self.game_board[self.height as usize - 1][column as usize - 1] =
            self.current_player.color();

        self.turns += 1;

        Ok(())
    }

    pub fn switch_player(&mut self) {
        let opposite_player = match self.current_player {
            Player::PlayerOne => match self.game_mode {
                GameMode::Multiplayer => Player::PlayerTwo,
                GameMode::SinglePlayer => Player::Computer,
            },
            _ => Player::PlayerOne,
        };

        self.current_player = opposite_player
    }

    pub fn are_four_connected(&self) -> Option<Player> {
        // check horizontal
        for row in 0..self.height {
            for col in 0..self.width - 3 {
                if self.game_board[row as usize][col as usize] != Piece::None
                    && self.game_board[row as usize][col as usize]
                        == self.game_board[row as usize][col as usize + 1]
                    && self.game_board[row as usize][col as usize]
                        == self.game_board[row as usize][col as usize + 2]
                    && self.game_board[row as usize][col as usize]
                        == self.game_board[row as usize][col as usize + 3]
                {
                    return Some(match self.game_board[row as usize][col as usize] {
                        Piece::Red => Player::PlayerOne,
                        Piece::Yellow => Player::PlayerTwo,
                        _ => panic!("Invalid piece"),
                    });
                }
            }
        }

        // check vertical
        for row in 0..self.height - 3 {
            for col in 0..self.width {
                if self.game_board[row as usize][col as usize] != Piece::None
                    && self.game_board[row as usize][col as usize]
                        == self.game_board[row as usize + 1][col as usize]
                    && self.game_board[row as usize][col as usize]
                        == self.game_board[row as usize + 2][col as usize]
                    && self.game_board[row as usize][col as usize]
                        == self.game_board[row as usize + 3][col as usize]
                {
                    return Some(match self.game_board[row as usize][col as usize] {
                        Piece::Red => Player::PlayerOne,
                        Piece::Yellow => Player::PlayerTwo,
                        _ => panic!("Invalid piece"),
                    });
                }
            }
        }

        // check diagonal
        for row in 0..self.height - 3 {
            for col in 0..self.width - 3 {
                if self.game_board[row as usize][col as usize] != Piece::None
                    && self.game_board[row as usize][col as usize]
                        == self.game_board[row as usize + 1][col as usize + 1]
                    && self.game_board[row as usize][col as usize]
                        == self.game_board[row as usize + 2][col as usize + 2]
                    && self.game_board[row as usize][col as usize]
                        == self.game_board[row as usize + 3][col as usize + 3]
                {
                    return Some(match self.game_board[row as usize][col as usize] {
                        Piece::Red => Player::PlayerOne,
                        Piece::Yellow => Player::PlayerTwo,
                        _ => panic!("Invalid piece"),
                    });
                }
            }
        }

        // check diagonal
        for row in 0..self.height - 3 {
            for col in 3..self.width {
                if self.game_board[row as usize][col as usize] != Piece::None
                    && self.game_board[row as usize][col as usize]
                        == self.game_board[row as usize + 1][col as usize - 1]
                    && self.game_board[row as usize][col as usize]
                        == self.game_board[row as usize + 2][col as usize - 2]
                    && self.game_board[row as usize][col as usize]
                        == self.game_board[row as usize + 3][col as usize - 3]
                {
                    return Some(match self.game_board[row as usize][col as usize] {
                        Piece::Red => Player::PlayerOne,
                        Piece::Yellow => Player::PlayerTwo,
                        _ => panic!("Invalid piece"),
                    });
                }
            }
        }

        None
    }

    pub fn debug(&self) {
        for row in &self.game_board {
            for piece in row {
                print!(" {} ", piece.to_string())
            }
            println!()
        }
    }

    pub fn draw_board_with_marker(&self, column: i32) {
        for i in 1..=self.width {
            if i == column {
                print!(
                    "  {}{}{}  ",
                    self.current_player.color().ansi_color(),
                    self.current_player.color().to_symbol(),
                    Color::Clear.to_ansi_color_code()
                );
            } else {
                print!("    ");
            }
        }
        println!();
        for row in &self.game_board {
            print!("+");
            for _ in 0..self.width {
                print!("---+");
            }
            println!();
            print!("|");
            for piece in row {
                print!(
                    " {}{}{} |",
                    piece.ansi_color(),
                    piece.to_symbol(),
                    Color::Clear.to_ansi_color_code()
                );
            }
            println!();
        }
        print!("+");
        for _ in 0..self.width {
            print!("---+");
        }
        println!();
        println!("Player: {}", self.current_player.to_string());
    }

    pub fn draw_board(&self) {
        for row in &self.game_board {
            print!("+");
            for _ in 0..self.width {
                print!("---+");
            }
            println!();
            print!("|");
            for piece in row {
                print!(
                    " {}{}{} |",
                    piece.ansi_color(),
                    piece.to_symbol(),
                    Color::Clear.to_ansi_color_code()
                );
            }
            println!();
        }
        print!("+");
        for _ in 0..self.width {
            print!("---+");
        }
        println!();
        println!("Player: {}", self.current_player.to_string());
    }
}

pub struct GameResult {
    turns: i32,
    winner: Option<Player>,
}

impl GameResult {
    pub fn new(turns: i32, winner: Option<Player>) -> GameResult {
        GameResult { turns, winner }
    }

    pub fn display(&self) {
        println!();
        match &self.winner {
            Some(winner) => {
                println!(
                    "{} won the game after {} turns!",
                    winner.to_string(),
                    self.turns
                )
            }
            None => {
                println!("The game ended in a draw after {} turns.", self.turns)
            }
        }
    }
}

pub fn run(game_mode: GameMode) {
    let game_result = match game_mode {
        GameMode::Multiplayer => play_multiplayer(),
        GameMode::SinglePlayer => play_singleplayer(),
    };

    game_result.display()
}
