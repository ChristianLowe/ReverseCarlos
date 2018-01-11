
pub enum Direction {
    Up,
    Down,
    Left,
    Right,

    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    /* Bit board information from https://www.hanshq.net/othello.html */
    // TODO try changing &self and *self to just "self"
    fn mask(&self) -> u64 {
        match *self {
            Direction::Up => 0xFFFFFFFFFFFFFFFF,
            Direction::Down => 0xFFFFFFFFFFFFFFFF,
            Direction::Left => 0xFEFEFEFEFEFEFEFE,
            Direction::Right => 0x7F7F7F7F7F7F7F7F,
            Direction::UpLeft => 0xFEFEFEFEFEFEFE00,
            Direction::UpRight => 0x7F7F7F7F7F7F7F00,
            Direction::DownLeft => 0x00FEFEFEFEFEFEFE,
            Direction::DownRight => 0x007F7F7F7F7F7F7F,
        }
    }

    fn shift_amount(&self) -> i8 {
        /* Negative numbers represent left shifting, positive numbers represent right shifting */
        match *self {
            Direction::Up => -8,
            Direction::Down => 8,
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::UpLeft => -9,
            Direction::UpRight => -7,
            Direction::DownLeft => 7,
            Direction::DownRight => 9,
        }
    }
}
