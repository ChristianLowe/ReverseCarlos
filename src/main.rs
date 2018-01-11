extern crate strum;
#[macro_use] extern crate strum_macros;
extern crate core;

use board::Board;
use board::Player;
use cell::Cell;

use std::io::{self, Write};

pub mod cell;
pub mod direction;
pub mod board;

static INPUT_ERROR_MSG: &'static str = "Please enter your input in algebraic format. E.g. 'A5'.";

fn main() {
    let mut board = Board::new();
    println!("{}", board);

    loop {
        print!("Possible moves: ");
        let valid_moves = board.get_valid_moves(Player::White);
        for valid_move in valid_moves {
            print!("{} ", valid_move);
        }
        print!("\n>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(len) => {
                if len != 3 {
                    println!("{}", INPUT_ERROR_MSG);
                    continue;
                }
            }
            Err(error) => panic!("error: {}", error)
        }

        input.pop(); // Discard the newline character

        let cell = Cell::from_algebraic_notation(&input);

        match cell {
            Ok(cell) => {
                let (row, column) = cell.get_grid_location();

                if board.is_valid_move(row, column, Player::White) {
                    board.make_move(row, column, Player::White);
                } else {
                    println!("Invalid move.");
                    continue;
                }
            },
            Err(_) => {
                println!("{}", INPUT_ERROR_MSG);
                continue;
            }
        }

        println!("{}", board);
    }
}
