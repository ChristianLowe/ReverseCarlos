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
    let mut player_turn = Player::White;
    let mut board = Board::new();

    loop {
        println!("{}", board);

        let valid_moves = board.get_valid_moves(player_turn);

        if valid_moves.len() == 0 {
            let white_score = board.get_score(Player::White);
            let black_score = board.get_score(Player::Black);

            print!("Game Over! ");
            if white_score > black_score {
                println!("White wins!");
            } else if black_score > white_score {
                println!("Black wins!");
            } else {
                println!("It's a tie!");
            }
            println!("\n===FINAL SCORE===");
            println!("White: {}, Black: {}", white_score, black_score);
            return;
        }

        match player_turn {
            Player::White => println!("White's turn! (You control the X's)."),
            Player::Black => println!("Black's turn! (You control the O's)."),
        }

        print!("Possible moves: ");
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

        match Cell::from_algebraic_notation(&input) {
            Ok(cell) => {
                let (row, column) = cell.get_grid_location();

                if board.is_valid_move(row, column, player_turn) {
                    board.make_move(row, column, player_turn);
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

        match player_turn {
            Player::White => player_turn = Player::Black,
            Player::Black => player_turn = Player::White,
        }
    }
}
