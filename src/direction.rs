use itertools::Itertools;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const DIRECTIONS: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];
    const REVERSED_DIRECTIONS: [Self; 4] = [Self::North, Self::West, Self::South, Self::East];

    pub fn cw(&self) -> Self {
        self.rotate(Self::DIRECTIONS, 1)
    }

    pub fn ccw(&self) -> Self {
        self.rotate(Self::REVERSED_DIRECTIONS, 1)
    }

    pub fn opposite(&self) -> Self {
        self.rotate(Self::DIRECTIONS, 2)
    }

    fn rotate(&self, directions: [Self; 4], times: usize) -> Self {
        let (i, _) = directions
            .into_iter()
            .find_position(|dir| dir == self)
            .unwrap();

        directions[(i + times) % 4]
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;

    #[test]
    fn cw() {
        assert_eq!(Direction::East, Direction::North.cw());
        assert_eq!(Direction::South, Direction::East.cw());
        assert_eq!(Direction::West, Direction::South.cw());
        assert_eq!(Direction::North, Direction::West.cw());
    }

    #[test]
    fn ccw() {
        assert_eq!(Direction::West, Direction::North.ccw());
        assert_eq!(Direction::North, Direction::East.ccw());
        assert_eq!(Direction::East, Direction::South.ccw());
        assert_eq!(Direction::South, Direction::West.ccw());
    }

    #[test]
    fn opposite() {
        assert_eq!(Direction::West, Direction::East.opposite());
        assert_eq!(Direction::North, Direction::South.opposite());
        assert_eq!(Direction::East, Direction::West.opposite());
        assert_eq!(Direction::South, Direction::North.opposite());
    }
}
