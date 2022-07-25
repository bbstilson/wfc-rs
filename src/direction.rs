#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    UpLeft,
    UpRight,
    Down,
    DownLeft,
    DownRight,
}

// impl Direction {
// pub fn flip(&self) -> Direction {
//     match self {
//         Direction::Up => Direction::Down,
//         Direction::Down => Direction::Up,
//         Direction::Left => Direction::Right,
//         Direction::Right => Direction::Left,
//         Direction::DownLeft => Direction::UpRight,
//         Direction::DownRight => Direction::UpLeft,
//         Direction::UpLeft => Direction::DownRight,
//         Direction::UpRight => Direction::DownLeft,
//     }
// }
// }
