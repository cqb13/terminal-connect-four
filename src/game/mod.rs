mod multiplayer;
mod singleplayer;

use self::multiplayer::play_multiplayer;
use self::singleplayer::play_singleplayer;

pub enum GameMode {
    SinglePlayer,
    Multiplayer,
}

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
}

pub struct GameResult {
    turns: i32,
    winner: Option<Player>,
}

impl GameResult {
    pub fn new() -> GameResult {
        GameResult {
            turns: 0,
            winner: None,
        }
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
