#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn idx(self) -> usize {
        match self {
            Direction::UP => 0,
            Direction::DOWN => 1,
            Direction::LEFT => 2,
            Direction::RIGHT => 3,
        }
    }
}

pub const ALL: [Direction; 4] = [
    Direction::UP,
    Direction::DOWN,
    Direction::LEFT,
    Direction::RIGHT,
];
