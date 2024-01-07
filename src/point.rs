use crate::direction::Direction;
use crate::direction::Direction::{East, North, South, West};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn adjacent(&self) -> [Self; 4] {
        [
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y - 1),
            Self::new(self.x, self.y + 1),
        ]
    }

    pub fn adjacent_in_directions(&self, directions: Vec<Direction>) -> Vec<Self> {
        directions
            .iter()
            .map(|direction| self.move_in(*direction))
            .collect()
    }

    pub fn move_in(&self, direction: Direction) -> Self {
        self.move_in_with_length(direction, 1)
    }

    pub fn manhattan_distance(&self, other: &Self) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    pub fn move_in_with_length(&self, direction: Direction, length: i32) -> Self {
        match direction {
            North => Self::new(self.x, self.y - length),
            East => Self::new(self.x + length, self.y),
            West => Self::new(self.x - length, self.y),
            South => Self::new(self.x, self.y + length),
        }
    }

    pub fn west(&self) -> Self {
        self.move_in(West)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;
    use crate::point::Point;

    #[test]
    fn adjacent_in_directions() {
        let point = Point::new(1, 1);

        assert_eq!(
            vec![Point::new(1, 0)],
            point.adjacent_in_directions(vec![Direction::North])
        );
        assert_eq!(
            vec![Point::new(1, 2)],
            point.adjacent_in_directions(vec![Direction::South])
        );
        assert_eq!(
            vec![Point::new(0, 1)],
            point.adjacent_in_directions(vec![Direction::West])
        );
        assert_eq!(
            vec![Point::new(2, 1)],
            point.adjacent_in_directions(vec![Direction::East])
        );

        assert_eq!(
            vec![Point::new(2, 1), Point::new(1, 2)],
            point.adjacent_in_directions(vec![Direction::East, Direction::South])
        )
    }
}
