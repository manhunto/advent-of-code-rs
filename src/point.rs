use crate::direction::Direction;
use crate::direction::Direction::{East, North, South, West};
use crate::range::Range;

#[derive(Debug,PartialEq,Copy,Clone,Eq,Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn adjacent(&self) -> Vec<Self> {
        vec![
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y - 1),
            Self::new(self.x, self.y + 1),
        ]
    }

    pub fn in_ranges(&self, x_range: Range, y_range: Range) -> bool {
        x_range.is_in_range(self.x as i64) && y_range.is_in_range(self.y as i64)
    }

    pub fn adjacent_in_directions(&self, directions: Vec<Direction>) -> Vec<Self> {
        directions
            .iter()
            .map(|direction| match direction {
                North => Self::new(self.x, self.y - 1),
                East => Self::new(self.x + 1, self.y),
                West => Self::new(self.x - 1, self.y),
                South => Self::new(self.x, self.y + 1),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;
    use crate::point::Point;

    #[test]
    fn adjacent_in_directions() {
        let point = Point::new(1, 1);

        assert_eq!(vec![Point::new(1, 0)], point.adjacent_in_directions(vec![Direction::North]));
        assert_eq!(vec![Point::new(1, 2)], point.adjacent_in_directions(vec![Direction::South]));
        assert_eq!(vec![Point::new(0, 1)], point.adjacent_in_directions(vec![Direction::West]));
        assert_eq!(vec![Point::new(2, 1)], point.adjacent_in_directions(vec![Direction::East]));

        assert_eq!(vec![Point::new(2, 1), Point::new(1, 2)], point.adjacent_in_directions(vec![Direction::East, Direction::South]))
    }
}