use std::fmt;

#[derive(Debug)]
pub struct Move {
    board_index: u8,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row = (self.board_index / 8) + ('1' as u8);
        let column = (self.board_index % 8) + ('A' as u8);
        write!(f, "{}{}", row, column)
    }
}

impl Move {
    pub fn from_index(board_index: u8) -> Move {
        debug_assert!(board_index < 64);

        Move { board_index }
    }

    pub fn from_grid(row: u8, column: u8) -> Move {
        debug_assert!(row < 8 && column < 8);

        let board_index = row * 8 + column;
        from_index(board_index)
    }

    pub fn from_algebraic_notation(input: &String) -> Move {
        debug_assert!(input.len() == 2);

        let bytes = input.to_uppercase().into_bytes();
        let row = bytes[1] - ('1' as u8);
        let column = bytes[0] - ('A' as u8);
        from_grid(row, column)
    }
}
