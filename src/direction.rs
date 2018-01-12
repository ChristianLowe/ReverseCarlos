
#[derive(EnumIter, Clone, Copy)]
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

pub enum ShiftType {
    Left(u8),
    Right(u8),
}

impl Direction {
    /* Bit board information from https://www.hanshq.net/othello.html */
    pub fn mask(self) -> u64 {
        match self {
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

    pub fn shift_type(self) -> ShiftType {
        match self {
            Direction::Up => ShiftType::Left(8),
            Direction::Down => ShiftType::Right(8),
            Direction::Left => ShiftType::Left(1),
            Direction::Right => ShiftType::Right(1),
            Direction::UpLeft => ShiftType::Left(9),
            Direction::UpRight => ShiftType::Left(7),
            Direction::DownLeft => ShiftType::Right(7),
            Direction::DownRight => ShiftType::Right(9),
        }
    }
}
