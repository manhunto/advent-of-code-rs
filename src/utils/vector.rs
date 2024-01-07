use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy, PartialOrd, Ord)]
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

    #[allow(dead_code)]
    pub fn rotate(&self, facing: Direction) -> Self {
        Self::new(self.position, facing)
    }

    pub fn rotate_cw(&self) -> Self {
        Self::new(self.position, self.facing.cw())
    }

    pub fn rotate_ccw(&self) -> Self {
        Self::new(self.position, self.facing.ccw())
    }
}
