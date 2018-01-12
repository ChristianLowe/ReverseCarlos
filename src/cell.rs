use std::fmt;

#[derive(Debug)]
pub struct Cell {
    board_index: u8,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row = ((self.board_index / 8) + ('1' as u8)) as char;
        let col = ((self.board_index % 8) + ('A' as u8)) as char;
        write!(f, "{}{}", col, row)
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

    pub fn from_algebraic_notation(input: &String) -> Result<Cell, ()> {
        debug_assert!(input.len() == 2);

        let bytes = input.to_uppercase().into_bytes();

        if bytes[0] > 'H' as u8 || bytes[0] < 'A' as u8
        || bytes[1] > '9' as u8 || bytes[1] < '1' as u8 {
            return Err(());
        }

        let row = bytes[1] - ('1' as u8);
        let column = bytes[0] - ('A' as u8);

        Ok(Cell::from_grid(row, column))
    }

    pub fn get_board_index(&self) -> u8 {
        self.board_index
    }

    pub fn get_grid_location(&self) -> (u8, u8) {
        let row = self.board_index / 8;
        let column = self.board_index % 8;

        (row, column)
    }
}
