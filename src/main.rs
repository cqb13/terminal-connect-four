pub mod display;
pub mod tui;

use crate::display::welcome;

fn main() {
    welcome::welcome();

}
