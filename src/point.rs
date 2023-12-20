use crate::direction::Direction;
use crate::direction::Direction::{East, North, South, West};
use crate::range::Range;

#[derive(Debug,PartialEq, Clone)]
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
        let mut vec: Vec<Self> = vec![];

        if directions.contains(&North) {
            vec.push(Self::new(self.x, self.y - 1));
        }

        if directions.contains(&East) {
            vec.push(Self::new(self.x + 1, self.y));
        }

        if directions.contains(&West) {
            vec.push(Self::new(self.x - 1, self.y));
        }

        if directions.contains(&South) {
            vec.push(Self::new(self.x, self.y + 1));
        }

        return vec.into_iter().collect();
    }
}