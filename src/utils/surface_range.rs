use crate::point::Point;
use crate::range::Range;

#[derive(Copy, Clone)]
pub struct SurfaceRange {
    x_range: Range,
    y_range: Range,
}

impl SurfaceRange {
    pub fn new(x_range: Range, y_range: Range) -> Self {
        Self {
            x_range,
            y_range,
        }
    }

    pub fn x(&self) -> Range {
        self.x_range
    }

    pub fn columns(&self) -> Range {
        self.x()
    }

    pub fn y(&self) -> Range {
        self.y_range
    }

    pub fn rows(&self) -> Range {
        self.y()
    }

    pub fn contains(&self, point: Point) -> bool {
        self.x_range.is_in_range(point.x as i64) && self.y_range.is_in_range(point.y as i64)
    }

    pub fn area(&self) -> usize {
        (self.x_range.len() * self.y_range.len()) as usize
    }
}