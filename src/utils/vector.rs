use crate::utils::direction::Direction;
use crate::utils::point::Point;

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

    pub fn backward(&self) -> Self {
        Self::new(self.position.move_in(self.facing.opposite()), self.facing)
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

    #[allow(dead_code)]
    pub fn opposite(&self) -> Self {
        Self::new(self.position, self.facing.opposite())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::direction::Direction;
    use crate::utils::point::Point;
    use crate::utils::vector::Vector;

    #[test]
    fn backward_test() {
        let point = Point::new(1, 1);
        let initial_vec = Vector::new(point, Direction::North);

        let backward = initial_vec.backward();
        assert_eq!(
            Vector::new(point.move_in(Direction::South), Direction::North),
            backward
        );
        assert_eq!(initial_vec, backward.forward());
    }
}
