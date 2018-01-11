extern crate strum;
#[macro_use] extern crate strum_macros;

use board::Board;
use board::Player;

pub mod cell;
pub mod direction;
pub mod board;

fn main() {
    let mut board = Board::new();
    println!("{}", board);
    board.make_move(4, 6, Player::White);
    println!("{}", board);
}
