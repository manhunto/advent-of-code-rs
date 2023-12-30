use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Vector {
    position: Point,
    facing: Direction,
}

impl Vector {
    pub fn new(position: Point, facing: Direction) -> Self {
        Self { position, facing }
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn facing(&self) -> Direction {
        self.facing
    }

    pub fn step(&self) -> Self {
        Self::new(self.position.move_in(self.facing), self.facing)
    }

    pub fn rotate(&self, facing: Direction) -> Self {
        Self::new(self.position, facing)
    }
}