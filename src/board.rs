use std::fmt;
use strum::IntoEnumIterator;

use cell::Cell;
use direction::{Direction, ShiftType};

pub struct Board {
    white_board: u64,
    black_board: u64,
}

pub enum CellState {
    White,
    Black,
    Empty,
}

pub enum Player {
    White,
    Black
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  A B C D E F G H\n")?;

        for row in 0..8 {
            write!(f, "{} ", row + 1)?;
            for column in 0..8 {
                let mask = Board::mask(row, column);

                if self.white_board & mask != 0 {
                    write!(f, "X ")?;
                } else if self.black_board & mask != 0 {
                    write!(f, "O ")?;
                } else {
                    write!(f, "- ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            white_board: 0b0,
            black_board: 0b0,
        };

        board.set_cell_state(3, 4, CellState::Black);
        board.set_cell_state(4, 3, CellState::Black);
        board.set_cell_state(3, 3, CellState::White);
        board.set_cell_state(4, 4, CellState::White);

        board
    }

    pub fn get_cell_state(&self, row: u8, column: u8) -> CellState {
        let mask = Board::mask(row, column);

        if (self.white_board & mask) != 0 {
            CellState::White
        } else if (self.black_board & mask) != 0 {
            CellState::Black
        } else {
            CellState::Empty
        }
    }

    pub fn set_cell_state(&mut self, row: u8, column: u8, cell_state: CellState) {
        let mask = Board::mask(row, column);

        self.white_board &= !mask;
        self.black_board &= !mask;

        match cell_state {
            CellState::White => self.white_board |= mask,
            CellState::Black => self.black_board |= mask,
            CellState::Empty => (),
        }
    }

    pub fn is_valid_move(&self, row: u8, column: u8, player: Player) -> bool {
        let mask = Board::mask(row, column);

        self.generate_move_board(player) & mask != 0
    }

    pub fn get_valid_moves(&self, player: Player) -> Vec<Cell> {
        let board = self.generate_move_board(player);
        let mut result = Vec::new();

        for index in 0..64 {
            let mask = (1 as u64) << index;

            if board & mask != 0 {
                result.push(Cell::from_index(index));
            }
        }

        result
    }

    pub fn get_score(&self, player: Player) -> u32 {
        match player {
            Player::White => self.white_board.count_ones(),
            Player::Black => self.black_board.count_ones(),
        }
    }

    pub fn make_move(&mut self, row: u8, column: u8, player: Player) {
        let board_index = row * 8 + column;

        self.resolve_move(board_index, player)
    }

    fn generate_move_board(&self, player: Player) -> u64 {
        let (friendly_board, enemy_board) = match player {
            Player::White => (self.white_board, self.black_board),
            Player::Black => (self.black_board, self.white_board),
        };

        debug_assert!((friendly_board & enemy_board) == 0, "Boards should be disjoint.");

        let empty_cells = !(friendly_board | enemy_board);
        let mut move_board = 0 as u64;

        for direction in Direction::iter() {
            // Get lines of adjacent enemy pieces going the current direction
            let mut adjacent_disks = Self::shift(friendly_board, direction) & enemy_board;
            for _ in 0..5 {
                adjacent_disks |= Self::shift(adjacent_disks, direction) & enemy_board;
            }

            // Empty cells adjacent to those are valid moves
            move_board |= Self::shift(adjacent_disks, direction) & empty_cells;
        }

        move_board
    }

    fn resolve_move(&mut self, board_index: u8, player: Player) {
        let (friendly_board, enemy_board) = match player {
            Player::White => (&mut self.white_board, &mut self.black_board),
            Player::Black => (&mut self.black_board, &mut self.white_board),
        };

        debug_assert!(board_index < 64, "Move must be within the board.");
        debug_assert!((*friendly_board & *enemy_board) == 0, "Boards should be disjoint.");

        let new_disk = (1 as u64) << board_index;
        let mut captured_disks = 0 as u64;

        debug_assert!((*friendly_board | *enemy_board) & new_disk == 0, "Target move isn't empty.");

        *friendly_board |= new_disk;

        for direction in Direction::iter() {
            // Get lines of enemy disks adjacent to the new piece
            let mut adjacent_disks = Self::shift(new_disk, direction) & *enemy_board;
            for _ in 0..5 {
                adjacent_disks |= Self::shift(adjacent_disks, direction) & *enemy_board;
            }

            // Capture resolution
            let bounding_disk = Self::shift(adjacent_disks, direction) & *friendly_board;
            captured_disks |= if bounding_disk == 0 { 0 } else { adjacent_disks };
        }

        debug_assert!(captured_disks != 0, "A valid move must capture disks.");

        *friendly_board ^= captured_disks;
        *enemy_board ^= captured_disks;
    }

    fn shift(pieces: u64, direction: Direction) -> u64 {
        match direction.shift_type() {
            ShiftType::Left(amount) => (pieces << amount) & direction.mask(),
            ShiftType::Right(amount) => (pieces >> amount) & direction.mask(),
        }
    }

    fn mask(row: u8, column: u8) -> u64 {
        debug_assert!(row < 8 && column < 8);

        (1 as u64) << (row * 8 + column)
    }
}
