use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Vector {
    position: Point,
    direction: Direction,
}

impl Vector {
    pub fn new(position: Point, direction: Direction) -> Self {
        Self { position, direction }
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn do_move(&self) -> Self {
        Self::new(self.position.move_in(self.direction), self.direction)
    }

    pub fn rotate(&self, direction: Direction) -> Self {
        Self::new(self.position, direction)
    }
}