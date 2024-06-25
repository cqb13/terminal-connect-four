pub mod display;
pub mod game;
pub mod tui;

use crate::display::welcome;
use crate::game::run;
use crate::game::GameMode;
use crate::tui::option_select::OptionSelect;

fn main() {
    welcome::welcome();

    let game_mode_option = OptionSelect::new()
        .set_title("Select a game mode:")
        .add_option("Single Player")
        .add_option("Multiplayer")
        .ask();

    let game_mode = match game_mode_option.as_str() {
        "Single Player" => GameMode::SinglePlayer,
        "Multiplayer" => GameMode::Multiplayer,
        _ => panic!("Invalid game mode selected"),
    };

    run(game_mode)
}
