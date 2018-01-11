use std::fmt;

#[derive(Debug)]
pub struct Cell {
    board_index: u8,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row = (self.board_index / 8) + ('1' as u8);
        let column = (self.board_index % 8) + ('A' as u8);
        write!(f, "{}{}", row, column)
    }
}

impl Cell {
    pub fn from_index(board_index: u8) -> Cell {
        debug_assert!(board_index < 64);

        Cell { board_index }
    }

    pub fn from_grid(row: u8, column: u8) -> Cell {
        debug_assert!(row < 8 && column < 8);

        let board_index = row * 8 + column;
        Cell::from_index(board_index)
    }

    pub fn from_algebraic_notation(input: &String) -> Cell {
        debug_assert!(input.len() == 2);

        let bytes = input.to_uppercase().into_bytes();
        let row = bytes[1] - ('1' as u8);
        let column = bytes[0] - ('A' as u8);
        Cell::from_grid(row, column)
    }
}
