use std::ops::Add;

use super::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub fn in_direction(self, direction: Direction) -> Vector2 {
        match direction {
            Direction::UP => self + Vector2 { x: 0, y: -1 },
            Direction::DOWN => self + Vector2 { x: 0, y: 1 },
            Direction::LEFT => self + Vector2 { x: -1, y: 0 },
            Direction::RIGHT => self + Vector2 { x: 1, y: 0 },
        }
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
