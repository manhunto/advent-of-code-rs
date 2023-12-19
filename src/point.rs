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
}