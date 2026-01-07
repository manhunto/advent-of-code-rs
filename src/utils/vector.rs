use crate::utils::direction::Direction;
use crate::utils::point::Point;
use std::fmt::{Display, Formatter};

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

    pub fn forward(&self) -> Self {
        Self::new(self.position.move_in(self.facing), self.facing)
    }

    pub fn forward_with_length(&self, length: isize) -> Self {
        Self::new(
            self.position.move_in_with_length(self.facing, length),
            self.facing,
        )
    }

    pub fn backward(&self) -> Self {
        Self::new(self.position.move_in(self.facing.opposite()), self.facing)
    }

    pub fn rotate(&self, facing: Direction) -> Self {
        Self::new(self.position, facing)
    }

    pub fn rotate_cw(&self) -> Self {
        Self::new(self.position, self.facing.cw())
    }

    pub fn rotate_ccw(&self) -> Self {
        Self::new(self.position, self.facing.ccw())
    }

    pub fn opposite(&self) -> Self {
        Self::new(self.position, self.facing.opposite())
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.position, self.facing)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::direction::Direction;
    use crate::utils::point::Point;
    use crate::utils::vector::Vector;

    #[test]
    fn backward_test() {
        let initial_vec = Vector::new(Point::new(1, 1), Direction::North);

        let backward = initial_vec.backward();

        assert_eq!(Vector::new(Point::new(1, 2), Direction::North), backward);
        assert_eq!(initial_vec, backward.forward());
    }

    #[test]
    fn forward_test() {
        let initial_vec = Vector::new(Point::new(1, 1), Direction::North);

        let forward = initial_vec.forward();

        assert_eq!(Vector::new(Point::new(1, 0), Direction::North), forward);
        assert_eq!(initial_vec, forward.backward());
    }
}
