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

    pub fn step(&mut self) {
        self.position = self.position.move_in(self.facing)
    }

    #[allow(dead_code)]
    pub fn rotate(&mut self, facing: Direction) {
        self.facing = facing
    }

    pub fn rotate_cw(&mut self) {
        self.facing = self.facing.cw()
    }

    pub fn rotate_ccw(&mut self) {
        self.facing = self.facing.ccw()
    }
}